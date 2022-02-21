import { Context } from "./session.ts";
import mainMenu from "./menu.ts";
import { State } from "./machine.ts";

export function welcome(ctx: Context) {
  ctx.reply("Ciao sono Elisa, pronta per aiutarti!");
}

export function showMenu(ctx: Context) {
  ctx.reply("Messaggio di prova con la keyboard", { reply_markup: mainMenu });
}

export function credits(ctx: Context) {
  ctx.reply(
    "Questo bot è fatto da quei meravigliosi tipi di Policumbent che amano le mucchette",
  );
}

export function cancel(ctx: Context) {
  ctx.session.state.toIdle();
  ctx.reply("Azione cancellata");
}

export function askToken(ctx: Context) {
  ctx.session.state.toSetToken();
  ctx.reply("Ok, madami un token.");
}

export function printToken(ctx: Context) {
  const token = ctx.session.apiToken;

  if (token) {
    ctx.reply(`Il tuo token è \`${ctx.session.apiToken}\`\\!`, {
      parse_mode: "MarkdownV2",
    });
  } else {
    ctx.reply("Ancora non mi hai mandato un token!");
  }
}

/**
 * Finite State Machine for emulating conversation with the bot
 */
export function machineReply(ctx: Context) {
  const state = ctx.session.state;

  if (state.is(State.SetToken)) {
    state.toIdle();

    const text = ctx.message?.text!;
    ctx.session.apiToken = text;

    printToken(ctx);
  } else {
    // idle
    showMenu(ctx);
  }
}
