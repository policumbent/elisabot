import { Context as _Context, SessionFlavor } from "./deps.ts";
import State from "./state.ts";

// Define the shape of our session.
export interface SessionData {
  apiToken: string | null;
  state: State;
}

// Flavor the context type to include sessions.
export type Context = _Context & SessionFlavor<SessionData>;

// Install session middleware, and define the initial session value.
export function initial(): SessionData {
  return { apiToken: null, state: new State() };
}
