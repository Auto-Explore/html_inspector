# html/rendering/non-replaced-elements/lists/lists-styles-quirks.html

Counts:
- errors: 1
- warnings: 100
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/rendering/non-replaced-elements/lists/lists-styles-quirks.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!-- This file is the same as lists-styles.html except:
- no doctype
- different title
- added quirks mode styles
-->
<title>quirks mode - default styles and preshints for ol, ul, menu, li, dir, dl, dt, dd</title>
<meta name="viewport" content="width=device-width">
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<script src="/html/rendering/support/test-ua-stylesheet.js"></script>
<style>
/* Specify this bogus namespace, so the rules in this stylesheet only apply to the `fakeClone`d elements in #refs, not the HTML elements in #tests. */
@namespace url(urn:not-html);

dir, dd, dl, dt, menu, ol, ul { display: block; }
li { display: list-item; text-align: match-parent; }

dir, dl, menu, ol, ul { margin-block-start: 1em; margin-block-end: 1em; }

:is(dir, dl, menu, ol, ul) :is(dir, dl, menu, ol, ul) {
  margin-block-start: 0; margin-block-end: 0;
}

dd { margin-inline-start: 40px; }
dir, menu, ol, ul { padding-inline-start: 40px; }

ol, ul, menu { counter-reset: list-item; }
ol { list-style-type: decimal; }

dir, menu, ul {
  list-style-type: disc;
}
:is(dir, menu, ol, ul) :is(dir, menu, ul) {
  list-style-type: circle;
}
:is(dir, menu, ol, ul) :is(dir, menu, ol, ul) :is(dir, menu, ul) {
  list-style-type: square;
}

/* preshints */
ol[type="1"], li[type="1"] { list-style-type: decimal; }
/* use classes due to lack of support for "s" annotation */
ol[class=type-a], li[class=type-a] { list-style-type: lower-alpha; }
ol[class=type-A], li[class=type-A] { list-style-type: upper-alpha; }
ol[class=type-i], li[class=type-i] { list-style-type: lower-roman; }
ol[class=type-I], li[class=type-I] { list-style-type: upper-roman; }
ul[type=none i], li[type=none i] { list-style-type: none; }
ul[type=disc i], li[type=disc i] { list-style-type: disc; }
ul[type=circle i], li[type=circle i] { list-style-type: circle; }
ul[type=square i], li[type=square i] { list-style-type: square; }

li[value="10"], li[value="10xyz"], li[value="10e10"] { counter-set: list-item 10; }
ol[start="10"], ol[start="10xyz"], ol[start="10e10"] { counter-reset: list-item 9; }
ol[reversed] { counter-reset: reversed(list-item); }
ol[reversed][start="20"], ol[reversed][start="20xyz"], ol[reversed][start="20e10"] { counter-reset: reversed(list-item) 21; }

/* dir="" */
[dir=ltr] { direction: ltr; }
[dir=rtl] { direction: rtl; }

/* quirks mode styles */
li { list-style-position: inside; }
li :is(dir, menu, ol, ul) { list-style-position: outside; }
:is(dir, menu, ol, ul) :is(dir, menu, ol, ul, li) { list-style-position: unset; }
</style>

<div id="log"></div>

<div id="tests">
  <li></li>
  <dir>
   <li></li>
  </dir>
  <dt></dt>
  <dd></dd>
  <dl>
   <dt></dt>
   <dd></dd>
  </dl>
  <menu>
   <li></li>
  </menu>
  <ol>
   <li></li>
  </ol>
  <ul>
   <li></li>
  </ul>
  <dir data-skip><dir></dir></dir>
  <dir data-skip><menu></menu></dir>
  <dir data-skip><ul></ul></dir>
  <menu data-skip><dir></dir></menu>
  <menu data-skip><menu></menu></menu>
  <menu data-skip><ul></ul></menu>
  <ol data-skip><dir></dir></ol>
  <ol data-skip><menu></menu></ol>
  <ol data-skip><ul></ul></ol>
  <ul data-skip><dir></dir></ul>
  <ul data-skip><menu></menu></ul>
  <ul data-skip><ul></ul></ul>

  <dir data-skip><dir data-skip><dir></dir></dir></dir>
  <dir data-skip><dir data-skip><menu></menu></dir></dir>
  <dir data-skip><dir data-skip><ul></ul></dir></dir>
  <dir data-skip><menu data-skip><dir></dir></menu></dir>
  <dir data-skip><menu data-skip><menu></menu></menu></dir>
  <dir data-skip><menu data-skip><ul></ul></menu></dir>
  <dir data-skip><ol data-skip><dir></dir></ol></dir>
  <dir data-skip><ol data-skip><menu></menu></ol></dir>
  <dir data-skip><ol data-skip><ul></ul></ol></dir>
  <dir data-skip><ul data-skip><dir></dir></ul></dir>
  <dir data-skip><ul data-skip><menu></menu></ul></dir>
  <dir data-skip><ul data-skip><ul></ul></ul></dir>

  <menu data-skip><dir data-skip><dir></dir></dir></menu>
  <menu data-skip><dir data-skip><menu></menu></dir></menu>
  <menu data-skip><dir data-skip><ul></ul></dir></menu>
  <menu data-skip><menu data-skip><dir></dir></menu></menu>
  <menu data-skip><menu data-skip><menu></menu></menu></menu>
  <menu data-skip><menu data-skip><ul></ul></menu></menu>
  <menu data-skip><ol data-skip><dir></dir></ol></menu>
  <menu data-skip><ol data-skip><menu></menu></ol></menu>
  <menu data-skip><ol data-skip><ul></ul></ol></menu>
  <menu data-skip><ul data-skip><dir></dir></ul></menu>
  <menu data-skip><ul data-skip><menu></menu></ul></menu>
  <menu data-skip><ul data-skip><ul></ul></ul></menu>

  <ol data-skip><dir data-skip><dir></dir></dir></ol>
  <ol data-skip><dir data-skip><menu></menu></dir></ol>
  <ol data-skip><dir data-skip><ul></ul></dir></ol>
  <ol data-skip><menu data-skip><dir></dir></menu></ol>
  <ol data-skip><menu data-skip><menu></menu></menu></ol>
  <ol data-skip><menu data-skip><ul></ul></menu></ol>
  <ol data-skip><ol data-skip><dir></dir></ol></ol>
  <ol data-skip><ol data-skip><menu></menu></ol></ol>
  <ol data-skip><ol data-skip><ul></ul></ol></ol>
  <ol data-skip><ul data-skip><dir></dir></ul></ol>
  <ol data-skip><ul data-skip><menu></menu></ul></ol>
  <ol data-skip><ul data-skip><ul></ul></ul></ol>

  <ul data-skip><dir data-skip><dir></dir></dir></ul>
  <ul data-skip><dir data-skip><menu></menu></dir></ul>
  <ul data-skip><dir data-skip><ul></ul></dir></ul>
  <ul data-skip><menu data-skip><dir></dir></menu></ul>
  <ul data-skip><menu data-skip><menu></menu></menu></ul>
  <ul data-skip><menu data-skip><ul></ul></menu></ul>
  <ul data-skip><ol data-skip><dir></dir></ol></ul>
  <ul data-skip><ol data-skip><menu></menu></ol></ul>
  <ul data-skip><ol data-skip><ul></ul></ol></ul>
  <ul data-skip><ul data-skip><dir></dir></ul></ul>
  <ul data-skip><ul data-skip><menu></menu></ul></ul>
  <ul data-skip><ul data-skip><ul></ul></ul></ul>

  <ol type="1"></ol>
  <ul type="1"></ul>
  <li type="1"></li>
  <ol type="a" class="type-a"></ol>
  <ul type="a" class="type-a"></ul>
  <li type="a" class="type-a"></li>
  <ol type="A" class="type-A"></ol>
  <ul type="A" class="type-A"></ul>
  <li type="A" class="type-A"></li>
  <ol type="i" class="type-i"></ol>
  <ul type="i" class="type-i"></ul>
  <li type="i" class="type-i"></li>
  <ol type="I" class="type-I"></ol>
  <ul type="I" class="type-I"></ul>
  <li type="I" class="type-I"></li>
  <ol type="none"></ol>
  <ul type="none"></ul>
  <li type="none"></li>
  <ol type="NONE"></ol>
  <ul type="NONE"></ul>
  <li type="NONE"></li>
  <ol type="disc"></ol>
  <ul type="disc"></ul>
  <li type="disc"></li>
  <ol type="DISC"></ol>
  <ul type="DISC"></ul>
  <li type="DISC"></li>
  <ol type="circle"></ol>
  <ul type="circle"></ul>
  <li type="circle"></li>
  <ol type="CIRCLE"></ol>
  <ul type="CIRCLE"></ul>
  <li type="CIRCLE"></li>
  <ol type="square"></ol>
  <ul type="square"></ul>
  <li type="square"></li>
  <ol type="SQUARE"></ol>
  <ul type="SQUARE"></ul>
  <li type="SQUARE"></li>

  <ol>
    <li value="10"></li>
    <li value="10xyz"></li>
    <li value="10e10"></li>
    <li value="xyz"></li>
  </ol>

  <ol start="10"><li></li></ol>
  <ol start="10xyz"><li></li></ol>
  <ol start="10e10"><li></li></ol>
  <ol start="xyz"><li></li></ol>
  <ol reversed><li></li></ol>
  <ol reversed start="20"><li></li></ol>
  <ol reversed start="20xyz"><li></li></ol>
  <ol reversed start="20e10"><li></li></ol>
  <ol reversed start="xyz"><li></li></ol>

  <ul data-skip dir="rtl"><li dir="ltr"></li></ul>
  <ul data-skip dir="ltr"><li dir="rtl"></li></ul>

</div>

<div id="refs"></div>

<script>
  const props = [
    'display',
    'margin-top',
    'margin-right',
    'margin-bottom',
    'margin-left',
    'padding-top',
    'padding-right',
    'padding-bottom',
    'padding-left',
    'list-style-type',
    'counter-set',
    'counter-reset',
    'counter-increment',
    'text-align',
    'list-style-position',
  ];
  runUAStyleTests(props);

</script>
```

```json
{
  "messages": [
    {
      "category": "Html",
      "code": "html.parser.start_tag_without_doctype",
      "message": "Start tag seen without seeing a doctype first. Expected “<!DOCTYPE html>”.",
      "severity": "Error",
      "span": {
        "byte_end": 125,
        "byte_start": 118,
        "col": 1,
        "line": 6
      }
    },
    {
      "category": "Html",
      "code": "html.li.parent.disallowed",
      "message": "Element “li” not allowed as child of “div” in this context.",
      "severity": "Warning",
      "span": {
        "byte_end": 2634,
        "byte_start": 2630,
        "col": 3,
        "line": 70
      }
    },
    {
      "category": "Html",
      "code": "html.element.dir.obsolete",
      "message": "The “dir” element is obsolete. Use the “ul” element instead.",
      "severity": "Warning",
      "span": {
        "byte_end": 2647,
        "byte_start": 2642,
        "col": 3,
        "line": 71
      }
    },
    {
      "category": "Html",
      "code": "html.li.parent.disallowed",
      "message": "Element “li” not allowed as child of “dir” in this context.",
      "severity": "Warning",
      "span": {
        "byte_end": 2655,
        "byte_start": 2651,
        "col": 4,
        "line": 72
      }
    },
    {
      "category": "Html",
      "code": "html.element.dir.obsolete",
      "message": "The “dir” element is obsolete. Use the “ul” element instead.",
      "severity": "Warning",
      "span": {
        "byte_end": 2840,
        "byte_start": 2825,
        "col": 3,
        "line": 89
      }
    },
    {
      "category": "Html",
      "code": "html.element.dir.obsolete",
      "message": "The “dir” element is obsolete. Use the “ul” element instead.",
      "severity": "Warning",
      "span": {
        "byte_end": 2845,
        "byte_start": 2840,
        "col": 18,
        "line": 89
      }
    },
    {
      "category": "Html",
      "code": "html.element.dir.obsolete",
      "message": "The “dir” element is obsolete. Use the “ul” element instead.",
      "severity": "Warning",
      "span": {
        "byte_end": 2875,
        "byte_start": 2860,
        "col": 3,
        "line": 90
      }
    },
    {
      "category": "Html",
      "code": "html.element.dir.obsolete",
      "message": "The “dir” element is obsolete. Use the “ul” element instead.",
      "severity": "Warning",
      "span": {
        "byte_end": 2912,
        "byte_start": 2897,
        "col": 3,
        "line": 91
      }
    },
    {
      "category": "Html",
      "code": "html.menu.child.not_allowed",
      "message": "Element “dir” not allowed as child of “menu” in this context.",
      "severity": "Warning",
      "span": {
        "byte_end": 2951,
        "byte_start": 2946,
        "col": 19,
        "line": 92
      }
    },
    {
      "category": "Html",
      "code": "html.element.dir.obsolete",
      "message": "The “dir” element is obsolete. Use the “ul” element instead.",
      "severity": "Warning",
      "span": {
        "byte_end": 2951,
        "byte_start": 2946,
        "col": 19,
        "line": 92
      }
    },
    {
      "category": "Html",
      "code": "html.menu.child.menu.not_allowed",
      "message": "Element “menu” not allowed as child of “menu” in this context.",
      "severity": "Warning",
      "span": {
        "byte_end": 2989,
        "byte_start": 2983,
        "col": 19,
        "line": 93
      }
    },
    {
      "category": "Html",
      "code": "html.menu.child.not_allowed",
      "message": "Element “ul” not allowed as child of “menu” in this context.",
      "severity": "Warning",
      "span": {
        "byte_end": 3026,
        "byte_start": 3022,
        "col": 19,
        "line": 94
      }
    },
    {
      "category": "Html",
      "code": "html.element.dir.obsolete",
      "message": "The “dir” element is obsolete. Use the “ul” element instead.",
      "severity": "Warning",
      "span": {
        "byte_end": 3060,
        "byte_start": 3055,
        "col": 17,
        "line": 95
      }
    },
    {
      "category": "Html",
      "code": "html.element.dir.obsolete",
      "message": "The “dir” element is obsolete. Use the “ul” element instead.",
      "severity": "Warning",
      "span": {
        "byte_end": 3159,
        "byte_start": 3154,
        "col": 17,
        "line": 98
      }
    },
    {
      "category": "Html",
      "code": "html.element.dir.obsolete",
      "message": "The “dir” element is obsolete. Use the “ul” element instead.",
      "severity": "Warning",
      "span": {
        "byte_end": 3255,
        "byte_start": 3240,
        "col": 3,
        "line": 102
      }
    },
    {
      "category": "Html",
      "code": "html.element.dir.obsolete",
      "message": "The “dir” element is obsolete. Use the “ul” element instead.",
      "severity": "Warning",
      "span": {
        "byte_end": 3270,
        "byte_start": 3255,
        "col": 18,
        "line": 102
      }
    },
    {
      "category": "Html",
      "code": "html.element.dir.obsolete",
      "message": "The “dir” element is obsolete. Use the “ul” element instead.",
      "severity": "Warning",
      "span": {
        "byte_end": 3275,
        "byte_start": 3270,
        "col": 33,
        "line": 102
      }
    },
    {
      "category": "Html",
      "code": "html.element.dir.obsolete",
      "message": "The “dir” element is obsolete. Use the “ul” element instead.",
      "severity": "Warning",
      "span": {
        "byte_end": 3311,
        "byte_start": 3296,
        "col": 3,
        "line": 103
      }
    },
    {
      "category": "Html",
      "code": "html.element.dir.obsolete",
      "message": "The “dir” element is obsolete. Use the “ul” element instead.",
      "severity": "Warning",
      "span": {
        "byte_end": 3326,
        "byte_start": 3311,
        "col": 18,
        "line": 103
      }
    },
    {
      "category": "Html",
      "code": "html.element.dir.obsolete",
      "message": "The “dir” element is obsolete. Use the “ul” element instead.",
      "severity": "Warning",
      "span": {
        "byte_end": 3369,
        "byte_start": 3354,
        "col": 3,
        "line": 104
      }
    },
    {
      "category": "Html",
      "code": "html.element.dir.obsolete",
      "message": "The “dir” element is obsolete. Use the “ul” element instead.",
      "severity": "Warning",
      "span": {
        "byte_end": 3384,
        "byte_start": 3369,
        "col": 18,
        "line": 104
      }
    },
    {
      "category": "Html",
      "code": "html.element.dir.obsolete",
      "message": "The “dir” element is obsolete. Use the “ul” element instead.",
      "severity": "Warning",
      "span": {
        "byte_end": 3423,
        "byte_start": 3408,
        "col": 3,
        "line": 105
      }
    },
    {
      "category": "Html",
      "code": "html.menu.child.not_allowed",
      "message": "Element “dir” not allowed as child of “menu” in this context.",
      "severity": "Warning",
      "span": {
        "byte_end": 3444,
        "byte_start": 3439,
        "col": 34,
        "line": 105
      }
    },
    {
      "category": "Html",
      "code": "html.element.dir.obsolete",
      "message": "The “dir” element is obsolete. Use the “ul” element instead.",
      "severity": "Warning",
      "span": {
        "byte_end": 3444,
        "byte_start": 3439,
        "col": 34,
        "line": 105
      }
    },
    {
      "category": "Html",
      "code": "html.element.dir.obsolete",
      "message": "The “dir” element is obsolete. Use the “ul” element instead.",
      "severity": "Warning",
      "span": {
        "byte_end": 3481,
        "byte_start": 3466,
        "col": 3,
        "line": 106
      }
    },
    {
      "category": "Html",
      "code": "html.menu.child.menu.not_allowed",
      "message": "Element “menu” not allowed as child of “menu” in this context.",
      "severity": "Warning",
      "span": {
        "byte_end": 3503,
        "byte_start": 3497,
        "col": 34,
        "line": 106
      }
    },
    {
      "category": "Html",
      "code": "html.element.dir.obsolete",
      "message": "The “dir” element is obsolete. Use the “ul” element instead.",
      "severity": "Warning",
      "span": {
        "byte_end": 3541,
        "byte_start": 3526,
        "col": 3,
        "line": 107
      }
    },
    {
      "category": "Html",
      "code": "html.menu.child.not_allowed",
      "message": "Element “ul” not allowed as child of “menu” in this context.",
      "severity": "Warning",
      "span": {
        "byte_end": 3561,
        "byte_start": 3557,
        "col": 34,
        "line": 107
      }
    },
    {
      "category": "Html",
      "code": "html.element.dir.obsolete",
      "message": "The “dir” element is obsolete. Use the “ul” element instead.",
      "severity": "Warning",
      "span": {
        "byte_end": 3597,
        "byte_start": 3582,
        "col": 3,
        "line": 108
      }
    },
    {
      "category": "Html",
      "code": "html.element.dir.obsolete",
      "message": "The “dir” element is obsolete. Use the “ul” element instead.",
      "severity": "Warning",
      "span": {
        "byte_end": 3616,
        "byte_start": 3611,
        "col": 32,
        "line": 108
      }
    },
    {
      "category": "Html",
      "code": "html.element.dir.obsolete",
      "message": "The “dir” element is obsolete. Use the “ul” element instead.",
      "severity": "Warning",
      "span": {
        "byte_end": 3651,
        "byte_start": 3636,
        "col": 3,
        "line": 109
      }
    },
    {
      "category": "Html",
      "code": "html.element.dir.obsolete",
      "message": "The “dir” element is obsolete. Use the “ul” element instead.",
      "severity": "Warning",
      "span": {
        "byte_end": 3707,
        "byte_start": 3692,
        "col": 3,
        "line": 110
      }
    },
    {
      "category": "Html",
      "code": "html.element.dir.obsolete",
      "message": "The “dir” element is obsolete. Use the “ul” element instead.",
      "severity": "Warning",
      "span": {
        "byte_end": 3759,
        "byte_start": 3744,
        "col": 3,
        "line": 111
      }
    },
    {
      "category": "Html",
      "code": "html.element.dir.obsolete",
      "message": "The “dir” element is obsolete. Use the “ul” element instead.",
      "severity": "Warning",
      "span": {
        "byte_end": 3778,
        "byte_start": 3773,
        "col": 32,
        "line": 111
      }
    },
    {
      "category": "Html",
      "code": "html.element.dir.obsolete",
      "message": "The “dir” element is obsolete. Use the “ul” element instead.",
      "severity": "Warning",
      "span": {
        "byte_end": 3813,
        "byte_start": 3798,
        "col": 3,
        "line": 112
      }
    },
    {
      "category": "Html",
      "code": "html.element.dir.obsolete",
      "message": "The “dir” element is obsolete. Use the “ul” element instead.",
      "severity": "Warning",
      "span": {
        "byte_end": 3869,
        "byte_start": 3854,
        "col": 3,
        "line": 113
      }
    },
    {
      "category": "Html",
      "code": "html.menu.child.not_allowed",
      "message": "Element “dir” not allowed as child of “menu” in this context.",
      "severity": "Warning",
      "span": {
        "byte_end": 3938,
        "byte_start": 3923,
        "col": 19,
        "line": 115
      }
    },
    {
      "category": "Html",
      "code": "html.element.dir.obsolete",
      "message": "The “dir” element is obsolete. Use the “ul” element instead.",
      "severity": "Warning",
      "span": {
        "byte_end": 3938,
        "byte_start": 3923,
        "col": 19,
        "line": 115
      }
    },
    {
      "category": "Html",
      "code": "html.element.dir.obsolete",
      "message": "The “dir” element is obsolete. Use the “ul” element instead.",
      "severity": "Warning",
      "span": {
        "byte_end": 3943,
        "byte_start": 3938,
        "col": 34,
        "line": 115
      }
    },
    {
      "category": "Html",
      "code": "html.menu.child.not_allowed",
      "message": "Element “dir” not allowed as child of “menu” in this context.",
      "severity": "Warning",
      "span": {
        "byte_end": 3996,
        "byte_start": 3981,
        "col": 19,
        "line": 116
      }
    },
    {
      "category": "Html",
      "code": "html.element.dir.obsolete",
      "message": "The “dir” element is obsolete. Use the “ul” element instead.",
      "severity": "Warning",
      "span": {
        "byte_end": 3996,
        "byte_start": 3981,
        "col": 19,
        "line": 116
      }
    },
    {
      "category": "Html",
      "code": "html.menu.child.not_allowed",
      "message": "Element “dir” not allowed as child of “menu” in this context.",
      "severity": "Warning",
      "span": {
        "byte_end": 4056,
        "byte_start": 4041,
        "col": 19,
        "line": 117
      }
    },
    {
      "category": "Html",
      "code": "html.element.dir.obsolete",
      "message": "The “dir” element is obsolete. Use the “ul” element instead.",
      "severity": "Warning",
      "span": {
        "byte_end": 4056,
        "byte_start": 4041,
        "col": 19,
        "line": 117
      }
    },
    {
      "category": "Html",
      "code": "html.menu.child.menu.not_allowed",
      "message": "Element “menu” not allowed as child of “menu” in this context.",
      "severity": "Warning",
      "span": {
        "byte_end": 4113,
        "byte_start": 4097,
        "col": 19,
        "line": 118
      }
    },
    {
      "category": "Html",
      "code": "html.menu.child.not_allowed",
      "message": "Element “dir” not allowed as child of “menu” in this context.",
      "severity": "Warning",
      "span": {
        "byte_end": 4118,
        "byte_start": 4113,
        "col": 35,
        "line": 118
      }
    },
    {
      "category": "Html",
      "code": "html.element.dir.obsolete",
      "message": "The “dir” element is obsolete. Use the “ul” element instead.",
      "severity": "Warning",
      "span": {
        "byte_end": 4118,
        "byte_start": 4113,
        "col": 35,
        "line": 118
      }
    },
    {
      "category": "Html",
      "code": "html.menu.child.menu.not_allowed",
      "message": "Element “menu” not allowed as child of “menu” in this context.",
      "severity": "Warning",
      "span": {
        "byte_end": 4173,
        "byte_start": 4157,
        "col": 19,
        "line": 119
      }
    },
    {
      "category": "Html",
      "code": "html.menu.child.menu.not_allowed",
      "message": "Element “menu” not allowed as child of “menu” in this context.",
      "severity": "Warning",
      "span": {
        "byte_end": 4179,
        "byte_start": 4173,
        "col": 35,
        "line": 119
      }
    },
    {
      "category": "Html",
      "code": "html.menu.child.menu.not_allowed",
      "message": "Element “menu” not allowed as child of “menu” in this context.",
      "severity": "Warning",
      "span": {
        "byte_end": 4235,
        "byte_start": 4219,
        "col": 19,
        "line": 120
      }
    },
    {
      "category": "Html",
      "code": "html.menu.child.not_allowed",
      "message": "Element “ul” not allowed as child of “menu” in this context.",
      "severity": "Warning",
      "span": {
        "byte_end": 4239,
        "byte_start": 4235,
        "col": 35,
        "line": 120
      }
    },
    {
      "category": "Html",
      "code": "html.menu.child.not_allowed",
      "message": "Element “ol” not allowed as child of “menu” in this context.",
      "severity": "Warning",
      "span": {
        "byte_end": 4291,
        "byte_start": 4277,
        "col": 19,
        "line": 121
      }
    },
    {
      "category": "Html",
      "code": "html.element.dir.obsolete",
      "message": "The “dir” element is obsolete. Use the “ul” element instead.",
      "severity": "Warning",
      "span": {
        "byte_end": 4296,
        "byte_start": 4291,
        "col": 33,
        "line": 121
      }
    },
    {
      "category": "Html",
      "code": "html.menu.child.not_allowed",
      "message": "Element “ol” not allowed as child of “menu” in this context.",
      "severity": "Warning",
      "span": {
        "byte_end": 4347,
        "byte_start": 4333,
        "col": 19,
        "line": 122
      }
    },
    {
      "category": "Html",
      "code": "html.menu.child.not_allowed",
      "message": "Element “ol” not allowed as child of “menu” in this context.",
      "severity": "Warning",
      "span": {
        "byte_end": 4405,
        "byte_start": 4391,
        "col": 19,
        "line": 123
      }
    },
    {
      "category": "Html",
      "code": "html.menu.child.not_allowed",
      "message": "Element “ul” not allowed as child of “menu” in this context.",
      "severity": "Warning",
      "span": {
        "byte_end": 4459,
        "byte_start": 4445,
        "col": 19,
        "line": 124
      }
    },
    {
      "category": "Html",
      "code": "html.element.dir.obsolete",
      "message": "The “dir” element is obsolete. Use the “ul” element instead.",
      "severity": "Warning",
      "span": {
        "byte_end": 4464,
        "byte_start": 4459,
        "col": 33,
        "line": 124
      }
    },
    {
      "category": "Html",
      "code": "html.menu.child.not_allowed",
      "message": "Element “ul” not allowed as child of “menu” in this context.",
      "severity": "Warning",
      "span": {
        "byte_end": 4515,
        "byte_start": 4501,
        "col": 19,
        "line": 125
      }
    },
    {
      "category": "Html",
      "code": "html.menu.child.not_allowed",
      "message": "Element “ul” not allowed as child of “menu” in this context.",
      "severity": "Warning",
      "span": {
        "byte_end": 4573,
        "byte_start": 4559,
        "col": 19,
        "line": 126
      }
    },
    {
      "category": "Html",
      "code": "html.element.dir.obsolete",
      "message": "The “dir” element is obsolete. Use the “ul” element instead.",
      "severity": "Warning",
      "span": {
        "byte_end": 4627,
        "byte_start": 4612,
        "col": 17,
        "line": 128
      }
    },
    {
      "category": "Html",
      "code": "html.element.dir.obsolete",
      "message": "The “dir” element is obsolete. Use the “ul” element instead.",
      "severity": "Warning",
      "span": {
        "byte_end": 4632,
        "byte_start": 4627,
        "col": 32,
        "line": 128
      }
    },
    {
      "category": "Html",
      "code": "html.element.dir.obsolete",
      "message": "The “dir” element is obsolete. Use the “ul” element instead.",
      "severity": "Warning",
      "span": {
        "byte_end": 4681,
        "byte_start": 4666,
        "col": 17,
        "line": 129
      }
    },
    {
      "category": "Html",
      "code": "html.element.dir.obsolete",
      "message": "The “dir” element is obsolete. Use the “ul” element instead.",
      "severity": "Warning",
      "span": {
        "byte_end": 4737,
        "byte_start": 4722,
        "col": 17,
        "line": 130
      }
    },
    {
      "category": "Html",
      "code": "html.menu.child.not_allowed",
      "message": "Element “dir” not allowed as child of “menu” in this context.",
      "severity": "Warning",
      "span": {
        "byte_end": 4795,
        "byte_start": 4790,
        "col": 33,
        "line": 131
      }
    },
    {
      "category": "Html",
      "code": "html.element.dir.obsolete",
      "message": "The “dir” element is obsolete. Use the “ul” element instead.",
      "severity": "Warning",
      "span": {
        "byte_end": 4795,
        "byte_start": 4790,
        "col": 33,
        "line": 131
      }
    },
    {
      "category": "Html",
      "code": "html.menu.child.menu.not_allowed",
      "message": "Element “menu” not allowed as child of “menu” in this context.",
      "severity": "Warning",
      "span": {
        "byte_end": 4852,
        "byte_start": 4846,
        "col": 33,
        "line": 132
      }
    },
    {
      "category": "Html",
      "code": "html.menu.child.not_allowed",
      "message": "Element “ul” not allowed as child of “menu” in this context.",
      "severity": "Warning",
      "span": {
        "byte_end": 4908,
        "byte_start": 4904,
        "col": 33,
        "line": 133
      }
    },
    {
      "category": "Html",
      "code": "html.element.dir.obsolete",
      "message": "The “dir” element is obsolete. Use the “ul” element instead.",
      "severity": "Warning",
      "span": {
        "byte_end": 4961,
        "byte_start": 4956,
        "col": 31,
        "line": 134
      }
    },
    {
      "category": "Html",
      "code": "html.element.dir.obsolete",
      "message": "The “dir” element is obsolete. Use the “ul” element instead.",
      "severity": "Warning",
      "span": {
        "byte_end": 5117,
        "byte_start": 5112,
        "col": 31,
        "line": 137
      }
    },
    {
      "category": "Html",
      "code": "html.element.dir.obsolete",
      "message": "The “dir” element is obsolete. Use the “ul” element instead.",
      "severity": "Warning",
      "span": {
        "byte_end": 5270,
        "byte_start": 5255,
        "col": 17,
        "line": 141
      }
    },
    {
      "category": "Html",
      "code": "html.element.dir.obsolete",
      "message": "The “dir” element is obsolete. Use the “ul” element instead.",
      "severity": "Warning",
      "span": {
        "byte_end": 5275,
        "byte_start": 5270,
        "col": 32,
        "line": 141
      }
    },
    {
      "category": "Html",
      "code": "html.element.dir.obsolete",
      "message": "The “dir” element is obsolete. Use the “ul” element instead.",
      "severity": "Warning",
      "span": {
        "byte_end": 5324,
        "byte_start": 5309,
        "col": 17,
        "line": 142
      }
    },
    {
      "category": "Html",
      "code": "html.element.dir.obsolete",
      "message": "The “dir” element is obsolete. Use the “ul” element instead.",
      "severity": "Warning",
      "span": {
        "byte_end": 5380,
        "byte_start": 5365,
        "col": 17,
        "line": 143
      }
    },
    {
      "category": "Html",
      "code": "html.menu.child.not_allowed",
      "message": "Element “dir” not allowed as child of “menu” in this context.",
      "severity": "Warning",
      "span": {
        "byte_end": 5438,
        "byte_start": 5433,
        "col": 33,
        "line": 144
      }
    },
    {
      "category": "Html",
      "code": "html.element.dir.obsolete",
      "message": "The “dir” element is obsolete. Use the “ul” element instead.",
      "severity": "Warning",
      "span": {
        "byte_end": 5438,
        "byte_start": 5433,
        "col": 33,
        "line": 144
      }
    },
    {
      "category": "Html",
      "code": "html.menu.child.menu.not_allowed",
      "message": "Element “menu” not allowed as child of “menu” in this context.",
      "severity": "Warning",
      "span": {
        "byte_end": 5495,
        "byte_start": 5489,
        "col": 33,
        "line": 145
      }
    },
    {
      "category": "Html",
      "code": "html.menu.child.not_allowed",
      "message": "Element “ul” not allowed as child of “menu” in this context.",
      "severity": "Warning",
      "span": {
        "byte_end": 5551,
        "byte_start": 5547,
        "col": 33,
        "line": 146
      }
    },
    {
      "category": "Html",
      "code": "html.element.dir.obsolete",
      "message": "The “dir” element is obsolete. Use the “ul” element instead.",
      "severity": "Warning",
      "span": {
        "byte_end": 5604,
        "byte_start": 5599,
        "col": 31,
        "line": 147
      }
    },
    {
      "category": "Html",
      "code": "html.element.dir.obsolete",
      "message": "The “dir” element is obsolete. Use the “ul” element instead.",
      "severity": "Warning",
      "span": {
        "byte_end": 5760,
        "byte_start": 5755,
        "col": 31,
        "line": 150
      }
    },
    {
      "category": "Html",
      "code": "html.li.parent.disallowed",
      "message": "Element “li” not allowed as child of “div” in this context.",
      "severity": "Warning",
      "span": {
        "byte_end": 5939,
        "byte_start": 5926,
        "col": 3,
        "line": 156
      }
    },
    {
      "category": "Html",
      "code": "html.li.parent.disallowed",
      "message": "Element “li” not allowed as child of “div” in this context.",
      "severity": "Warning",
      "span": {
        "byte_end": 6047,
        "byte_start": 6019,
        "col": 3,
        "line": 159
      }
    },
    {
      "category": "Html",
      "code": "html.li.parent.disallowed",
      "message": "Element “li” not allowed as child of “div” in this context.",
      "severity": "Warning",
      "span": {
        "byte_end": 6155,
        "byte_start": 6127,
        "col": 3,
        "line": 162
      }
    },
    {
      "category": "Html",
      "code": "html.li.parent.disallowed",
      "message": "Element “li” not allowed as child of “div” in this context.",
      "severity": "Warning",
      "span": {
        "byte_end": 6263,
        "byte_start": 6235,
        "col": 3,
        "line": 165
      }
    },
    {
      "category": "Html",
      "code": "html.li.parent.disallowed",
      "message": "Element “li” not allowed as child of “div” in this context.",
      "severity": "Warning",
      "span": {
        "byte_end": 6371,
        "byte_start": 6343,
        "col": 3,
        "line": 168
      }
    },
    {
      "category": "Html",
      "code": "html.li.parent.disallowed",
      "message": "Element “li” not allowed as child of “div” in this context.",
      "severity": "Warning",
      "span": {
        "byte_end": 6443,
        "byte_start": 6427,
        "col": 3,
        "line": 171
      }
    },
    {
      "category": "Html",
      "code": "html.li.parent.disallowed",
      "message": "Element “li” not allowed as child of “div” in this context.",
      "severity": "Warning",
      "span": {
        "byte_end": 6515,
        "byte_start": 6499,
        "col": 3,
        "line": 174
      }
    },
    {
      "category": "Html",
      "code": "html.li.parent.disallowed",
      "message": "Element “li” not allowed as child of “div” in this context.",
      "severity": "Warning",
      "span": {
        "byte_end": 6587,
        "byte_start": 6571,
        "col": 3,
        "line": 177
      }
    },
    {
      "category": "Html",
      "code": "html.li.parent.disallowed",
      "message": "Element “li” not allowed as child of “div” in this context.",
      "severity": "Warning",
      "span": {
        "byte_end": 6659,
        "byte_start": 6643,
        "col": 3,
        "line": 180
      }
    },
    {
      "category": "Html",
      "code": "html.li.parent.disallowed",
      "message": "Element “li” not allowed as child of “div” in this context.",
      "severity": "Warning",
      "span": {
        "byte_end": 6737,
        "byte_start": 6719,
        "col": 3,
        "line": 183
      }
    },
    {
      "category": "Html",
      "code": "html.li.parent.disallowed",
      "message": "Element “li” not allowed as child of “div” in this context.",
      "severity": "Warning",
      "span": {
        "byte_end": 6815,
        "byte_start": 6797,
        "col": 3,
        "line": 186
      }
    },
    {
      "category": "Html",
      "code": "html.li.parent.disallowed",
      "message": "Element “li” not allowed as child of “div” in this context.",
      "severity": "Warning",
      "span": {
        "byte_end": 6893,
        "byte_start": 6875,
        "col": 3,
        "line": 189
      }
    },
    {
      "category": "Html",
      "code": "html.li.parent.disallowed",
      "message": "Element “li” not allowed as child of “div” in this context.",
      "severity": "Warning",
      "span": {
        "byte_end": 6971,
        "byte_start": 6953,
        "col": 3,
        "line": 192
      }
    },
    {
      "category": "Html",
      "code": "html.li.value.invalid",
      "message": "Bad value “10xyz” for attribute “value” on element “li”.",
      "severity": "Warning",
      "span": {
        "byte_end": 7032,
        "byte_start": 7014,
        "col": 5,
        "line": 196
      }
    },
    {
      "category": "Html",
      "code": "html.li.value.invalid",
      "message": "Bad value “10e10” for attribute “value” on element “li”.",
      "severity": "Warning",
      "span": {
        "byte_end": 7060,
        "byte_start": 7042,
        "col": 5,
        "line": 197
      }
    },
    {
      "category": "Html",
      "code": "html.li.value.invalid",
      "message": "Bad value “xyz” for attribute “value” on element “li”.",
      "severity": "Warning",
      "span": {
        "byte_end": 7086,
        "byte_start": 7070,
        "col": 5,
        "line": 198
      }
    },
    {
      "category": "Html",
      "code": "html.ol.start.invalid",
      "message": "Bad value “10xyz” for attribute “start” on element “ol”.",
      "severity": "Warning",
      "span": {
        "byte_end": 7153,
        "byte_start": 7135,
        "col": 3,
        "line": 202
      }
    },
    {
      "category": "Html",
      "code": "html.ol.start.invalid",
      "message": "Bad value “10e10” for attribute “start” on element “ol”.",
      "severity": "Warning",
      "span": {
        "byte_end": 7188,
        "byte_start": 7170,
        "col": 3,
        "line": 203
      }
    },
    {
      "category": "Html",
      "code": "html.ol.start.invalid",
      "message": "Bad value “xyz” for attribute “start” on element “ol”.",
      "severity": "Warning",
      "span": {
        "byte_end": 7221,
        "byte_start": 7205,
        "col": 3,
        "line": 204
      }
    },
    {
      "category": "Html",
      "code": "html.ol.start.invalid",
      "message": "Bad value “20xyz” for attribute “start” on element “ol”.",
      "severity": "Warning",
      "span": {
        "byte_end": 7336,
        "byte_start": 7309,
        "col": 3,
        "line": 207
      }
    },
    {
      "category": "Html",
      "code": "html.ol.start.invalid",
      "message": "Bad value “20e10” for attribute “start” on element “ol”.",
      "severity": "Warning",
      "span": {
        "byte_end": 7380,
        "byte_start": 7353,
        "col": 3,
        "line": 208
      }
    },
    {
      "category": "Html",
      "code": "html.ol.start.invalid",
      "message": "Bad value “xyz” for attribute “start” on element “ol”.",
      "severity": "Warning",
      "span": {
        "byte_end": 7422,
        "byte_start": 7397,
        "col": 3,
        "line": 209
      }
    },
    {
      "category": "I18n",
      "code": "i18n.lang.missing",
      "message": "Consider adding a “lang” attribute to the “html” start tag to declare the language of this document.",
      "severity": "Warning",
      "span": null
    }
  ],
  "source_name": "html/rendering/non-replaced-elements/lists/lists-styles-quirks.html"
}
```
