import { Menu } from "./deps.ts";
import { Context } from "./session.ts";
import { askToken, credits, printToken } from "./messages.ts";

/**
 * Main menu used as root of inlineKeybord
 */
const mainMenu = new Menu<Context>("inline")
  .text("get token", printToken)
  .text("set token", askToken)
  .submenu("Credits", "credits-menu");

const settings = new Menu<Context>("credits-menu")
  .text("Show Credits", credits)
  .back("Go Back");

mainMenu.register(settings);

export default mainMenu;
