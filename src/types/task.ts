export interface TaskInput {
  title: string;
  requester: string;
  due: string;
  body: string;
  custom_properties?: Record<string, string | number | boolean | null>;
}

export interface CreateTaskResult {
  success: boolean;
  file_path: string;
  filename: string;
}
