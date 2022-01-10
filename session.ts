import { Context, SessionFlavor } from "./deps.ts";

// Define the shape of our session.
export interface SessionData {
  apiToken: string;
}

// Flavor the context type to include sessions.
export type ApiContext = Context & SessionFlavor<SessionData>;

// Install session middleware, and define the initial session value.
export function initial(): SessionData {
  return { apiToken: "" };
}
