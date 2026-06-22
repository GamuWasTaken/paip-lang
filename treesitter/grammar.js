/**
 * @file Paip grammar for tree-sitter
 * @author GamuWasTaken
 * @license MIT
 */

/// <reference types="tree-sitter-cli/dsl" />
// @ts-check

export default grammar({
  name: "paip",

  rules: {
    source_file: $ => repeat(seq($.expression, ';')),

    expression: $ => choice(
      $.define,
      $.assign,
      $.access,
      $.fncall,
      $.return,
      $.name,
      $.block
    ),

    typing: $ => choice(
      $.name,
      $.access,
      seq($.block, '>', $.block)
    ),
    define: $ => seq(
      choice($.name, $.access), ':', choice($.typing)
    ),
    assign: $ => seq(
      choice($.define, $.name, $.access), '=', choice($.block, $.name)
    ),
    access: $ => seq(
      choice($.name, $.access, '@'), '::', choice($.name, $.block)
    ),
    fncall: $ => seq(
      choice($.name, $.access), '.', $.name, optional($.block)
    ),
    return: $ => seq('@>', choice($.access, $.block, $.name)),

    name: _ => /[a-zA-Z0-9_]+/,
    block: $ => seq(
      '[',
      optional(choice(
        repeat(seq($.expression, ';')),
        $.expression
      )),
      ']'
    ),


    comment: _ => token(seq('#', /.*/))
  },

  extras: $ => [
    /\s/,
    $.comment
  ]
});
