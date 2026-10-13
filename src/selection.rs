use crate::models::{Faction, GameData};

/// The character selection the user has committed to so far,
/// kept as ids so it survives `GameData` reloads.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SelectionState {
    pub faction: Faction,
    pub race_id: String,
    pub class_id: String,
}

impl SelectionState {
    /// Reconcile the selection after the user switches faction.
    ///
    /// A race still available in the new faction is kept (dual-faction
    /// Skyborne survives the switch); its class is kept only when it is
    /// playable by that race in the new faction, otherwise it is coerced
    /// to the first playable class. A race unavailable in the new faction
    /// resets the whole selection to the new faction's defaults.
    pub fn after_faction_change(&self, data: &GameData, new_faction: Faction) -> SelectionState {
        let race_still_available = data
            .get_race_by_id(&self.race_id)
            .map(|r| r.is_available_for(new_faction))
            .unwrap_or(false);

        if !race_still_available {
            let race = data.default_race_for_faction(new_faction);
            let class_id = data
                .default_class_for_race(new_faction, &race.id)
                .map(|c| c.id)
                .unwrap_or_default();
            return SelectionState {
                faction: new_faction,
                race_id: race.id,
                class_id,
            };
        }

        let class_still_valid =
            data.is_class_available_for_race(new_faction, &self.race_id, &self.class_id);
        let class_id = if class_still_valid {
            self.class_id.clone()
        } else {
            data.default_class_for_race(new_faction, &self.race_id)
                .map(|c| c.id)
                .unwrap_or_else(|| self.class_id.clone())
        };

        SelectionState {
            faction: new_faction,
            race_id: self.race_id.clone(),
            class_id,
        }
    }
}
