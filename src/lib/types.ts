export interface CheatSheetEntry {
  id: string;
  name: string;
  action: string;
  command?: string[];
  commandText?: string;
  commandSequence?: string[][];
  aliases?: string[];
  tags?: string[];
}

export interface CheatSheet {
  id: string;
  name: string;
  icon?: string;
  tags?: string[];
  entries: CheatSheetEntry[];
}

export interface CheatSheetsPayload {
  directory: string;
  sheets: CheatSheet[];
}
