import { Menu } from "./deps.ts";
import { ApiContext } from "./session.ts";

class State {
  token: boolean;

  constructor() {
    this.token = false;
  }
}

export const state = new State();

/**
 * Main menu used as root of inlineKeybord
 */
const mainMenu = new Menu<ApiContext>("inline")
  .text("get token", (ctx) =>
    ctx.reply(`Your token is: ${ctx.session.apiToken}.`)
  )
  .text("set token", (ctx) => {
    ctx.reply("Ok, send me your new token.");
    state.token = true;
  })
  .submenu("Credits", "credits-menu");

const settings = new Menu<ApiContext>("credits-menu")
  .text("Show Credits", (ctx) => ctx.reply("Powered by grammY"))
  .back("Go Back");

mainMenu.register(settings);

export default mainMenu;
