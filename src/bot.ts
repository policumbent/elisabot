import { Bot, session } from "../deps.ts";
import { Context, initial } from "./session.ts";
import mainMenu from "./menu.ts";

import * as messages from "./messages.ts";

// Create an instance of the `Bot`
const token = Deno.env.get("BOT_TOKEN") || "";
const bot = new Bot<Context>(token);

bot.use(session({ initial }));
bot.use(mainMenu);

/* Commands */

// Helper for commands
bot.api.setMyCommands([
  { command: "start", description: "Avvia il bot" },
  { command: "show", description: "Mostra una keyboard con i comandi" },
  { command: "credits", description: "Display i credits del bot" },
  { command: "cancel", description: "Cancella l'operazione corrente" },
]);

// Handle commands
bot.command("start", messages.welcome);
bot.command("show", messages.showMenu);
bot.command("credits", messages.credits);
bot.command("cancel", messages.cancel);

// Handle normal messages with a FSM
bot.on("message:text", messages.machineReply);

// Start the bot.
bot.start();
