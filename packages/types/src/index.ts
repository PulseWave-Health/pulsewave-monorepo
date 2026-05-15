export type ValidationStatus = "pending" | "validated" | "rejected";

export interface CustomerRecord {
  address: string;
  dataHash: string;
  status: ValidationStatus;
  timestamp: number;
}

export interface SubmitPayload {
  address: string;
  dataHash: string;
}

export interface ValidatePayload {
  address: string;
  approved: boolean;
}

export interface ApiResponse<T> {
  success: boolean;
  data?: T;
  error?: string;
}
