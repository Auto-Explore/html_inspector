# html/rendering/non-replaced-elements/lists/lists-styles.html

Counts:
- errors: 0
- warnings: 100
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/rendering/non-replaced-elements/lists/lists-styles.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!doctype html>
<title>default styles and preshints for ol, ul, menu, li, dir, dl, dt, dd</title>
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
      "code": "html.li.parent.disallowed",
      "message": "Element “li” not allowed as child of “div” in this context.",
      "severity": "Warning",
      "span": {
        "byte_end": 2314,
        "byte_start": 2310,
        "col": 3,
        "line": 61
      }
    },
    {
      "category": "Html",
      "code": "html.element.dir.obsolete",
      "message": "The “dir” element is obsolete. Use the “ul” element instead.",
      "severity": "Warning",
      "span": {
        "byte_end": 2327,
        "byte_start": 2322,
        "col": 3,
        "line": 62
      }
    },
    {
      "category": "Html",
      "code": "html.li.parent.disallowed",
      "message": "Element “li” not allowed as child of “dir” in this context.",
      "severity": "Warning",
      "span": {
        "byte_end": 2335,
        "byte_start": 2331,
        "col": 4,
        "line": 63
      }
    },
    {
      "category": "Html",
      "code": "html.element.dir.obsolete",
      "message": "The “dir” element is obsolete. Use the “ul” element instead.",
      "severity": "Warning",
      "span": {
        "byte_end": 2520,
        "byte_start": 2505,
        "col": 3,
        "line": 80
      }
    },
    {
      "category": "Html",
      "code": "html.element.dir.obsolete",
      "message": "The “dir” element is obsolete. Use the “ul” element instead.",
      "severity": "Warning",
      "span": {
        "byte_end": 2525,
        "byte_start": 2520,
        "col": 18,
        "line": 80
      }
    },
    {
      "category": "Html",
      "code": "html.element.dir.obsolete",
      "message": "The “dir” element is obsolete. Use the “ul” element instead.",
      "severity": "Warning",
      "span": {
        "byte_end": 2555,
        "byte_start": 2540,
        "col": 3,
        "line": 81
      }
    },
    {
      "category": "Html",
      "code": "html.element.dir.obsolete",
      "message": "The “dir” element is obsolete. Use the “ul” element instead.",
      "severity": "Warning",
      "span": {
        "byte_end": 2592,
        "byte_start": 2577,
        "col": 3,
        "line": 82
      }
    },
    {
      "category": "Html",
      "code": "html.menu.child.not_allowed",
      "message": "Element “dir” not allowed as child of “menu” in this context.",
      "severity": "Warning",
      "span": {
        "byte_end": 2631,
        "byte_start": 2626,
        "col": 19,
        "line": 83
      }
    },
    {
      "category": "Html",
      "code": "html.element.dir.obsolete",
      "message": "The “dir” element is obsolete. Use the “ul” element instead.",
      "severity": "Warning",
      "span": {
        "byte_end": 2631,
        "byte_start": 2626,
        "col": 19,
        "line": 83
      }
    },
    {
      "category": "Html",
      "code": "html.menu.child.menu.not_allowed",
      "message": "Element “menu” not allowed as child of “menu” in this context.",
      "severity": "Warning",
      "span": {
        "byte_end": 2669,
        "byte_start": 2663,
        "col": 19,
        "line": 84
      }
    },
    {
      "category": "Html",
      "code": "html.menu.child.not_allowed",
      "message": "Element “ul” not allowed as child of “menu” in this context.",
      "severity": "Warning",
      "span": {
        "byte_end": 2706,
        "byte_start": 2702,
        "col": 19,
        "line": 85
      }
    },
    {
      "category": "Html",
      "code": "html.element.dir.obsolete",
      "message": "The “dir” element is obsolete. Use the “ul” element instead.",
      "severity": "Warning",
      "span": {
        "byte_end": 2740,
        "byte_start": 2735,
        "col": 17,
        "line": 86
      }
    },
    {
      "category": "Html",
      "code": "html.element.dir.obsolete",
      "message": "The “dir” element is obsolete. Use the “ul” element instead.",
      "severity": "Warning",
      "span": {
        "byte_end": 2839,
        "byte_start": 2834,
        "col": 17,
        "line": 89
      }
    },
    {
      "category": "Html",
      "code": "html.element.dir.obsolete",
      "message": "The “dir” element is obsolete. Use the “ul” element instead.",
      "severity": "Warning",
      "span": {
        "byte_end": 2935,
        "byte_start": 2920,
        "col": 3,
        "line": 93
      }
    },
    {
      "category": "Html",
      "code": "html.element.dir.obsolete",
      "message": "The “dir” element is obsolete. Use the “ul” element instead.",
      "severity": "Warning",
      "span": {
        "byte_end": 2950,
        "byte_start": 2935,
        "col": 18,
        "line": 93
      }
    },
    {
      "category": "Html",
      "code": "html.element.dir.obsolete",
      "message": "The “dir” element is obsolete. Use the “ul” element instead.",
      "severity": "Warning",
      "span": {
        "byte_end": 2955,
        "byte_start": 2950,
        "col": 33,
        "line": 93
      }
    },
    {
      "category": "Html",
      "code": "html.element.dir.obsolete",
      "message": "The “dir” element is obsolete. Use the “ul” element instead.",
      "severity": "Warning",
      "span": {
        "byte_end": 2991,
        "byte_start": 2976,
        "col": 3,
        "line": 94
      }
    },
    {
      "category": "Html",
      "code": "html.element.dir.obsolete",
      "message": "The “dir” element is obsolete. Use the “ul” element instead.",
      "severity": "Warning",
      "span": {
        "byte_end": 3006,
        "byte_start": 2991,
        "col": 18,
        "line": 94
      }
    },
    {
      "category": "Html",
      "code": "html.element.dir.obsolete",
      "message": "The “dir” element is obsolete. Use the “ul” element instead.",
      "severity": "Warning",
      "span": {
        "byte_end": 3049,
        "byte_start": 3034,
        "col": 3,
        "line": 95
      }
    },
    {
      "category": "Html",
      "code": "html.element.dir.obsolete",
      "message": "The “dir” element is obsolete. Use the “ul” element instead.",
      "severity": "Warning",
      "span": {
        "byte_end": 3064,
        "byte_start": 3049,
        "col": 18,
        "line": 95
      }
    },
    {
      "category": "Html",
      "code": "html.element.dir.obsolete",
      "message": "The “dir” element is obsolete. Use the “ul” element instead.",
      "severity": "Warning",
      "span": {
        "byte_end": 3103,
        "byte_start": 3088,
        "col": 3,
        "line": 96
      }
    },
    {
      "category": "Html",
      "code": "html.menu.child.not_allowed",
      "message": "Element “dir” not allowed as child of “menu” in this context.",
      "severity": "Warning",
      "span": {
        "byte_end": 3124,
        "byte_start": 3119,
        "col": 34,
        "line": 96
      }
    },
    {
      "category": "Html",
      "code": "html.element.dir.obsolete",
      "message": "The “dir” element is obsolete. Use the “ul” element instead.",
      "severity": "Warning",
      "span": {
        "byte_end": 3124,
        "byte_start": 3119,
        "col": 34,
        "line": 96
      }
    },
    {
      "category": "Html",
      "code": "html.element.dir.obsolete",
      "message": "The “dir” element is obsolete. Use the “ul” element instead.",
      "severity": "Warning",
      "span": {
        "byte_end": 3161,
        "byte_start": 3146,
        "col": 3,
        "line": 97
      }
    },
    {
      "category": "Html",
      "code": "html.menu.child.menu.not_allowed",
      "message": "Element “menu” not allowed as child of “menu” in this context.",
      "severity": "Warning",
      "span": {
        "byte_end": 3183,
        "byte_start": 3177,
        "col": 34,
        "line": 97
      }
    },
    {
      "category": "Html",
      "code": "html.element.dir.obsolete",
      "message": "The “dir” element is obsolete. Use the “ul” element instead.",
      "severity": "Warning",
      "span": {
        "byte_end": 3221,
        "byte_start": 3206,
        "col": 3,
        "line": 98
      }
    },
    {
      "category": "Html",
      "code": "html.menu.child.not_allowed",
      "message": "Element “ul” not allowed as child of “menu” in this context.",
      "severity": "Warning",
      "span": {
        "byte_end": 3241,
        "byte_start": 3237,
        "col": 34,
        "line": 98
      }
    },
    {
      "category": "Html",
      "code": "html.element.dir.obsolete",
      "message": "The “dir” element is obsolete. Use the “ul” element instead.",
      "severity": "Warning",
      "span": {
        "byte_end": 3277,
        "byte_start": 3262,
        "col": 3,
        "line": 99
      }
    },
    {
      "category": "Html",
      "code": "html.element.dir.obsolete",
      "message": "The “dir” element is obsolete. Use the “ul” element instead.",
      "severity": "Warning",
      "span": {
        "byte_end": 3296,
        "byte_start": 3291,
        "col": 32,
        "line": 99
      }
    },
    {
      "category": "Html",
      "code": "html.element.dir.obsolete",
      "message": "The “dir” element is obsolete. Use the “ul” element instead.",
      "severity": "Warning",
      "span": {
        "byte_end": 3331,
        "byte_start": 3316,
        "col": 3,
        "line": 100
      }
    },
    {
      "category": "Html",
      "code": "html.element.dir.obsolete",
      "message": "The “dir” element is obsolete. Use the “ul” element instead.",
      "severity": "Warning",
      "span": {
        "byte_end": 3387,
        "byte_start": 3372,
        "col": 3,
        "line": 101
      }
    },
    {
      "category": "Html",
      "code": "html.element.dir.obsolete",
      "message": "The “dir” element is obsolete. Use the “ul” element instead.",
      "severity": "Warning",
      "span": {
        "byte_end": 3439,
        "byte_start": 3424,
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
        "byte_end": 3458,
        "byte_start": 3453,
        "col": 32,
        "line": 102
      }
    },
    {
      "category": "Html",
      "code": "html.element.dir.obsolete",
      "message": "The “dir” element is obsolete. Use the “ul” element instead.",
      "severity": "Warning",
      "span": {
        "byte_end": 3493,
        "byte_start": 3478,
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
        "byte_end": 3549,
        "byte_start": 3534,
        "col": 3,
        "line": 104
      }
    },
    {
      "category": "Html",
      "code": "html.menu.child.not_allowed",
      "message": "Element “dir” not allowed as child of “menu” in this context.",
      "severity": "Warning",
      "span": {
        "byte_end": 3618,
        "byte_start": 3603,
        "col": 19,
        "line": 106
      }
    },
    {
      "category": "Html",
      "code": "html.element.dir.obsolete",
      "message": "The “dir” element is obsolete. Use the “ul” element instead.",
      "severity": "Warning",
      "span": {
        "byte_end": 3618,
        "byte_start": 3603,
        "col": 19,
        "line": 106
      }
    },
    {
      "category": "Html",
      "code": "html.element.dir.obsolete",
      "message": "The “dir” element is obsolete. Use the “ul” element instead.",
      "severity": "Warning",
      "span": {
        "byte_end": 3623,
        "byte_start": 3618,
        "col": 34,
        "line": 106
      }
    },
    {
      "category": "Html",
      "code": "html.menu.child.not_allowed",
      "message": "Element “dir” not allowed as child of “menu” in this context.",
      "severity": "Warning",
      "span": {
        "byte_end": 3676,
        "byte_start": 3661,
        "col": 19,
        "line": 107
      }
    },
    {
      "category": "Html",
      "code": "html.element.dir.obsolete",
      "message": "The “dir” element is obsolete. Use the “ul” element instead.",
      "severity": "Warning",
      "span": {
        "byte_end": 3676,
        "byte_start": 3661,
        "col": 19,
        "line": 107
      }
    },
    {
      "category": "Html",
      "code": "html.menu.child.not_allowed",
      "message": "Element “dir” not allowed as child of “menu” in this context.",
      "severity": "Warning",
      "span": {
        "byte_end": 3736,
        "byte_start": 3721,
        "col": 19,
        "line": 108
      }
    },
    {
      "category": "Html",
      "code": "html.element.dir.obsolete",
      "message": "The “dir” element is obsolete. Use the “ul” element instead.",
      "severity": "Warning",
      "span": {
        "byte_end": 3736,
        "byte_start": 3721,
        "col": 19,
        "line": 108
      }
    },
    {
      "category": "Html",
      "code": "html.menu.child.menu.not_allowed",
      "message": "Element “menu” not allowed as child of “menu” in this context.",
      "severity": "Warning",
      "span": {
        "byte_end": 3793,
        "byte_start": 3777,
        "col": 19,
        "line": 109
      }
    },
    {
      "category": "Html",
      "code": "html.menu.child.not_allowed",
      "message": "Element “dir” not allowed as child of “menu” in this context.",
      "severity": "Warning",
      "span": {
        "byte_end": 3798,
        "byte_start": 3793,
        "col": 35,
        "line": 109
      }
    },
    {
      "category": "Html",
      "code": "html.element.dir.obsolete",
      "message": "The “dir” element is obsolete. Use the “ul” element instead.",
      "severity": "Warning",
      "span": {
        "byte_end": 3798,
        "byte_start": 3793,
        "col": 35,
        "line": 109
      }
    },
    {
      "category": "Html",
      "code": "html.menu.child.menu.not_allowed",
      "message": "Element “menu” not allowed as child of “menu” in this context.",
      "severity": "Warning",
      "span": {
        "byte_end": 3853,
        "byte_start": 3837,
        "col": 19,
        "line": 110
      }
    },
    {
      "category": "Html",
      "code": "html.menu.child.menu.not_allowed",
      "message": "Element “menu” not allowed as child of “menu” in this context.",
      "severity": "Warning",
      "span": {
        "byte_end": 3859,
        "byte_start": 3853,
        "col": 35,
        "line": 110
      }
    },
    {
      "category": "Html",
      "code": "html.menu.child.menu.not_allowed",
      "message": "Element “menu” not allowed as child of “menu” in this context.",
      "severity": "Warning",
      "span": {
        "byte_end": 3915,
        "byte_start": 3899,
        "col": 19,
        "line": 111
      }
    },
    {
      "category": "Html",
      "code": "html.menu.child.not_allowed",
      "message": "Element “ul” not allowed as child of “menu” in this context.",
      "severity": "Warning",
      "span": {
        "byte_end": 3919,
        "byte_start": 3915,
        "col": 35,
        "line": 111
      }
    },
    {
      "category": "Html",
      "code": "html.menu.child.not_allowed",
      "message": "Element “ol” not allowed as child of “menu” in this context.",
      "severity": "Warning",
      "span": {
        "byte_end": 3971,
        "byte_start": 3957,
        "col": 19,
        "line": 112
      }
    },
    {
      "category": "Html",
      "code": "html.element.dir.obsolete",
      "message": "The “dir” element is obsolete. Use the “ul” element instead.",
      "severity": "Warning",
      "span": {
        "byte_end": 3976,
        "byte_start": 3971,
        "col": 33,
        "line": 112
      }
    },
    {
      "category": "Html",
      "code": "html.menu.child.not_allowed",
      "message": "Element “ol” not allowed as child of “menu” in this context.",
      "severity": "Warning",
      "span": {
        "byte_end": 4027,
        "byte_start": 4013,
        "col": 19,
        "line": 113
      }
    },
    {
      "category": "Html",
      "code": "html.menu.child.not_allowed",
      "message": "Element “ol” not allowed as child of “menu” in this context.",
      "severity": "Warning",
      "span": {
        "byte_end": 4085,
        "byte_start": 4071,
        "col": 19,
        "line": 114
      }
    },
    {
      "category": "Html",
      "code": "html.menu.child.not_allowed",
      "message": "Element “ul” not allowed as child of “menu” in this context.",
      "severity": "Warning",
      "span": {
        "byte_end": 4139,
        "byte_start": 4125,
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
        "byte_end": 4144,
        "byte_start": 4139,
        "col": 33,
        "line": 115
      }
    },
    {
      "category": "Html",
      "code": "html.menu.child.not_allowed",
      "message": "Element “ul” not allowed as child of “menu” in this context.",
      "severity": "Warning",
      "span": {
        "byte_end": 4195,
        "byte_start": 4181,
        "col": 19,
        "line": 116
      }
    },
    {
      "category": "Html",
      "code": "html.menu.child.not_allowed",
      "message": "Element “ul” not allowed as child of “menu” in this context.",
      "severity": "Warning",
      "span": {
        "byte_end": 4253,
        "byte_start": 4239,
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
        "byte_end": 4307,
        "byte_start": 4292,
        "col": 17,
        "line": 119
      }
    },
    {
      "category": "Html",
      "code": "html.element.dir.obsolete",
      "message": "The “dir” element is obsolete. Use the “ul” element instead.",
      "severity": "Warning",
      "span": {
        "byte_end": 4312,
        "byte_start": 4307,
        "col": 32,
        "line": 119
      }
    },
    {
      "category": "Html",
      "code": "html.element.dir.obsolete",
      "message": "The “dir” element is obsolete. Use the “ul” element instead.",
      "severity": "Warning",
      "span": {
        "byte_end": 4361,
        "byte_start": 4346,
        "col": 17,
        "line": 120
      }
    },
    {
      "category": "Html",
      "code": "html.element.dir.obsolete",
      "message": "The “dir” element is obsolete. Use the “ul” element instead.",
      "severity": "Warning",
      "span": {
        "byte_end": 4417,
        "byte_start": 4402,
        "col": 17,
        "line": 121
      }
    },
    {
      "category": "Html",
      "code": "html.menu.child.not_allowed",
      "message": "Element “dir” not allowed as child of “menu” in this context.",
      "severity": "Warning",
      "span": {
        "byte_end": 4475,
        "byte_start": 4470,
        "col": 33,
        "line": 122
      }
    },
    {
      "category": "Html",
      "code": "html.element.dir.obsolete",
      "message": "The “dir” element is obsolete. Use the “ul” element instead.",
      "severity": "Warning",
      "span": {
        "byte_end": 4475,
        "byte_start": 4470,
        "col": 33,
        "line": 122
      }
    },
    {
      "category": "Html",
      "code": "html.menu.child.menu.not_allowed",
      "message": "Element “menu” not allowed as child of “menu” in this context.",
      "severity": "Warning",
      "span": {
        "byte_end": 4532,
        "byte_start": 4526,
        "col": 33,
        "line": 123
      }
    },
    {
      "category": "Html",
      "code": "html.menu.child.not_allowed",
      "message": "Element “ul” not allowed as child of “menu” in this context.",
      "severity": "Warning",
      "span": {
        "byte_end": 4588,
        "byte_start": 4584,
        "col": 33,
        "line": 124
      }
    },
    {
      "category": "Html",
      "code": "html.element.dir.obsolete",
      "message": "The “dir” element is obsolete. Use the “ul” element instead.",
      "severity": "Warning",
      "span": {
        "byte_end": 4641,
        "byte_start": 4636,
        "col": 31,
        "line": 125
      }
    },
    {
      "category": "Html",
      "code": "html.element.dir.obsolete",
      "message": "The “dir” element is obsolete. Use the “ul” element instead.",
      "severity": "Warning",
      "span": {
        "byte_end": 4797,
        "byte_start": 4792,
        "col": 31,
        "line": 128
      }
    },
    {
      "category": "Html",
      "code": "html.element.dir.obsolete",
      "message": "The “dir” element is obsolete. Use the “ul” element instead.",
      "severity": "Warning",
      "span": {
        "byte_end": 4950,
        "byte_start": 4935,
        "col": 17,
        "line": 132
      }
    },
    {
      "category": "Html",
      "code": "html.element.dir.obsolete",
      "message": "The “dir” element is obsolete. Use the “ul” element instead.",
      "severity": "Warning",
      "span": {
        "byte_end": 4955,
        "byte_start": 4950,
        "col": 32,
        "line": 132
      }
    },
    {
      "category": "Html",
      "code": "html.element.dir.obsolete",
      "message": "The “dir” element is obsolete. Use the “ul” element instead.",
      "severity": "Warning",
      "span": {
        "byte_end": 5004,
        "byte_start": 4989,
        "col": 17,
        "line": 133
      }
    },
    {
      "category": "Html",
      "code": "html.element.dir.obsolete",
      "message": "The “dir” element is obsolete. Use the “ul” element instead.",
      "severity": "Warning",
      "span": {
        "byte_end": 5060,
        "byte_start": 5045,
        "col": 17,
        "line": 134
      }
    },
    {
      "category": "Html",
      "code": "html.menu.child.not_allowed",
      "message": "Element “dir” not allowed as child of “menu” in this context.",
      "severity": "Warning",
      "span": {
        "byte_end": 5118,
        "byte_start": 5113,
        "col": 33,
        "line": 135
      }
    },
    {
      "category": "Html",
      "code": "html.element.dir.obsolete",
      "message": "The “dir” element is obsolete. Use the “ul” element instead.",
      "severity": "Warning",
      "span": {
        "byte_end": 5118,
        "byte_start": 5113,
        "col": 33,
        "line": 135
      }
    },
    {
      "category": "Html",
      "code": "html.menu.child.menu.not_allowed",
      "message": "Element “menu” not allowed as child of “menu” in this context.",
      "severity": "Warning",
      "span": {
        "byte_end": 5175,
        "byte_start": 5169,
        "col": 33,
        "line": 136
      }
    },
    {
      "category": "Html",
      "code": "html.menu.child.not_allowed",
      "message": "Element “ul” not allowed as child of “menu” in this context.",
      "severity": "Warning",
      "span": {
        "byte_end": 5231,
        "byte_start": 5227,
        "col": 33,
        "line": 137
      }
    },
    {
      "category": "Html",
      "code": "html.element.dir.obsolete",
      "message": "The “dir” element is obsolete. Use the “ul” element instead.",
      "severity": "Warning",
      "span": {
        "byte_end": 5284,
        "byte_start": 5279,
        "col": 31,
        "line": 138
      }
    },
    {
      "category": "Html",
      "code": "html.element.dir.obsolete",
      "message": "The “dir” element is obsolete. Use the “ul” element instead.",
      "severity": "Warning",
      "span": {
        "byte_end": 5440,
        "byte_start": 5435,
        "col": 31,
        "line": 141
      }
    },
    {
      "category": "Html",
      "code": "html.li.parent.disallowed",
      "message": "Element “li” not allowed as child of “div” in this context.",
      "severity": "Warning",
      "span": {
        "byte_end": 5619,
        "byte_start": 5606,
        "col": 3,
        "line": 147
      }
    },
    {
      "category": "Html",
      "code": "html.li.parent.disallowed",
      "message": "Element “li” not allowed as child of “div” in this context.",
      "severity": "Warning",
      "span": {
        "byte_end": 5727,
        "byte_start": 5699,
        "col": 3,
        "line": 150
      }
    },
    {
      "category": "Html",
      "code": "html.li.parent.disallowed",
      "message": "Element “li” not allowed as child of “div” in this context.",
      "severity": "Warning",
      "span": {
        "byte_end": 5835,
        "byte_start": 5807,
        "col": 3,
        "line": 153
      }
    },
    {
      "category": "Html",
      "code": "html.li.parent.disallowed",
      "message": "Element “li” not allowed as child of “div” in this context.",
      "severity": "Warning",
      "span": {
        "byte_end": 5943,
        "byte_start": 5915,
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
        "byte_end": 6051,
        "byte_start": 6023,
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
        "byte_end": 6123,
        "byte_start": 6107,
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
        "byte_end": 6195,
        "byte_start": 6179,
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
        "byte_end": 6267,
        "byte_start": 6251,
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
        "byte_end": 6339,
        "byte_start": 6323,
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
        "byte_end": 6417,
        "byte_start": 6399,
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
        "byte_end": 6495,
        "byte_start": 6477,
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
        "byte_end": 6573,
        "byte_start": 6555,
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
        "byte_end": 6651,
        "byte_start": 6633,
        "col": 3,
        "line": 183
      }
    },
    {
      "category": "Html",
      "code": "html.li.value.invalid",
      "message": "Bad value “10xyz” for attribute “value” on element “li”.",
      "severity": "Warning",
      "span": {
        "byte_end": 6712,
        "byte_start": 6694,
        "col": 5,
        "line": 187
      }
    },
    {
      "category": "Html",
      "code": "html.li.value.invalid",
      "message": "Bad value “10e10” for attribute “value” on element “li”.",
      "severity": "Warning",
      "span": {
        "byte_end": 6740,
        "byte_start": 6722,
        "col": 5,
        "line": 188
      }
    },
    {
      "category": "Html",
      "code": "html.li.value.invalid",
      "message": "Bad value “xyz” for attribute “value” on element “li”.",
      "severity": "Warning",
      "span": {
        "byte_end": 6766,
        "byte_start": 6750,
        "col": 5,
        "line": 189
      }
    },
    {
      "category": "Html",
      "code": "html.ol.start.invalid",
      "message": "Bad value “10xyz” for attribute “start” on element “ol”.",
      "severity": "Warning",
      "span": {
        "byte_end": 6833,
        "byte_start": 6815,
        "col": 3,
        "line": 193
      }
    },
    {
      "category": "Html",
      "code": "html.ol.start.invalid",
      "message": "Bad value “10e10” for attribute “start” on element “ol”.",
      "severity": "Warning",
      "span": {
        "byte_end": 6868,
        "byte_start": 6850,
        "col": 3,
        "line": 194
      }
    },
    {
      "category": "Html",
      "code": "html.ol.start.invalid",
      "message": "Bad value “xyz” for attribute “start” on element “ol”.",
      "severity": "Warning",
      "span": {
        "byte_end": 6901,
        "byte_start": 6885,
        "col": 3,
        "line": 195
      }
    },
    {
      "category": "Html",
      "code": "html.ol.start.invalid",
      "message": "Bad value “20xyz” for attribute “start” on element “ol”.",
      "severity": "Warning",
      "span": {
        "byte_end": 7016,
        "byte_start": 6989,
        "col": 3,
        "line": 198
      }
    },
    {
      "category": "Html",
      "code": "html.ol.start.invalid",
      "message": "Bad value “20e10” for attribute “start” on element “ol”.",
      "severity": "Warning",
      "span": {
        "byte_end": 7060,
        "byte_start": 7033,
        "col": 3,
        "line": 199
      }
    },
    {
      "category": "Html",
      "code": "html.ol.start.invalid",
      "message": "Bad value “xyz” for attribute “start” on element “ol”.",
      "severity": "Warning",
      "span": {
        "byte_end": 7102,
        "byte_start": 7077,
        "col": 3,
        "line": 200
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
  "source_name": "html/rendering/non-replaced-elements/lists/lists-styles.html"
}
```
