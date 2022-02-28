export enum State {
  Idle,
  SetToken,
}

class Machine {
  private state = State.Idle;

  getState() {
    return this.state;
  }

  setState(newState: State) {
    this.state = newState;
  }

  is(state: State): boolean {
    return this.state === state;
  }

  // shortcut for setting state

  toIdle() {
    this.setState(State.Idle);
  }

  toSetToken() {
    this.setState(State.SetToken);
  }
}

export default Machine;
