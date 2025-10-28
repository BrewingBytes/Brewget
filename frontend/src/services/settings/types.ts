export interface Settings {
  user_id: string;
  language: string;
  currency: string;
  alarm_set: boolean;
  alarm_time: string;
  alarm_offset_minutes: number;
  night_mode: boolean;
}

export interface UpdateSettings {
  language?: string;
  currency?: string;
  alarm_set?: boolean;
  alarm_time?: string;
  alarm_offset_minutes?: number;
  night_mode?: boolean;
}
