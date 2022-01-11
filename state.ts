const stateStr = ["idle", "set-token"];

class State {
  private id = 0;

  // todo: auto generate methods for changing state

  toIdle() {
    this.id = 0;
  }

  isIdle() {
    return this.id === 0;
  }

  toSetToken() {
    this.id = 1;
  }

  isSetToken() {
    return this.id === 1;
  }

  getState() {
    return stateStr[this.id];
  }

  changeState(newState: number | string) {
    if (typeof newState === "number" && newState < stateStr.length) {
      this.id = newState;
    } else if (
      typeof newState === "string" &&
      stateStr.includes(newState)
    ) {
      this.id = stateStr.indexOf(newState);
    }
  }
}

export default State;
