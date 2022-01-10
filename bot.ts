import { Bot, session } from "./deps.ts";
import { ApiContext, initial } from "./session.ts";
import mainMenu, { state } from "./menu.ts";

const token = Deno.env.get("BOT_TOKEN") || "";

// Create an instance of the `Bot` class and pass your authentication token to it.
const bot = new Bot<ApiContext>(token);

bot.use(session({ initial }));
bot.use(mainMenu);

/* Commands */

bot.api.setMyCommands([
  { command: "start", description: "Avvia il bot" },
  { command: "show", description: "Mostra una keyboard con tutti i comandi" },
  { command: "token", description: "get token" },
  { command: "newtoken", description: "set token" },
  // { command: "login", description: "Permette l'accesso alle api" },
  // { command: "admin", description: "Display dei comandi degli admin" },
  // { command: "credits", description: "Display i credits del bot" },
  // { command: "cancel", description: "Cancella l'operazione corrente" },
]);

// Handle the /start command.
bot.command("start", (ctx) => ctx.reply("Welcome! Up and running."));
bot.command("show", (ctx) => ctx.reply("prova", { reply_markup: mainMenu }));

// Handle other messages.
bot.on("message:text", (ctx) => {
  if (state.token) {
    const tokenStr = ctx.message.text;
    ctx.session.apiToken = tokenStr;
    state.token = false;
    ctx.reply(`Ok, \`${tokenStr}\` is your new token\\!`, {
      parse_mode: "MarkdownV2",
    });
  } else {
    ctx.reply("Got another message!");
  }
});

// Start the bot.
bot.start();
