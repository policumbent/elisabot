import { Context as _Context, SessionFlavor } from "../deps.ts";
import Machine from "./machine.ts";

export interface SessionData {
  apiToken: string | null;

  // finite state machine for reply actions
  state: Machine;
}

// Flavor the context type to include sessions.
export type Context = _Context & SessionFlavor<SessionData>;

// Install session middleware, and define the initial session value.
export function initial(): SessionData {
  return { apiToken: null, state: new Machine() };
}
