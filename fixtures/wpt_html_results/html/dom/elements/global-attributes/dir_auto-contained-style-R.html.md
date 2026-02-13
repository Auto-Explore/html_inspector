# html/dom/elements/global-attributes/dir_auto-contained-style-R.html

Counts:
- errors: 0
- warnings: 5
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/dom/elements/global-attributes/dir_auto-contained-style-R.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<html>
  <head>
    <meta charset="utf-8" />
    <title>HTML Test: dir=auto, start with style, then R</title>
    <link rel="match" href="dir_auto-contained-style-R-ref.html" />
    <link rel="author" title="Matitiahu Allouche" href="mailto:matitiahu.allouche@google.com" />
    <link rel="author" title="Oren Roth" href="mailto:oren.roth@gmail.com" />
    <link rel="author" title="HTML5 bidi test WG" href="mailto:html5bidi@googlegroups.com" />
    <link rel="help" href="https://html.spec.whatwg.org/multipage/#the-dir-attribute" />
    <meta name="assert" content="
      When dir='auto', the direction is set according to the first strong character
      of descendant text while ignoring descendant style elements.
      In this test, it is the Hebrew letter Alef, thus the direction must be
      resolved as RTL." />
    <style>
      input, textarea {
        font-size:1em;
      }
      body {
        font-size:2em;
      }
      .test, .ref {
        border: medium solid gray;
        width: 400px;
        margin: 20px;
      }
      .comments {
        display: none;
      }
    </style>
  </head>
  <body>
    <div class="instructions"><p>Test passes if the two boxes below look exactly the same.</p></div>
    <div class="comments">
      Key to entities used below:
      &#x05D0; - The Hebrew letter Alef (strongly RTL).
      &#x05D1; - The Hebrew letter Bet (strongly RTL).
      &#x05D2; - The Hebrew letter Gimel (strongly RTL).
    </div>
    <div class="test">
      <div dir="ltr">
        <div dir="auto"><style>body {color:black;}</style>&#x05D0;&#x05D1;&#x05D2;ABC.</div>
      </div>
      <div dir="rtl">
        <div dir="auto"><style>body {color:black;}</style>&#x05D0;&#x05D1;&#x05D2;ABC.</div>
      </div>
    </div>
    <div class="ref">
      <div dir="ltr">
        <div dir="rtl"><style>body {color:black;}</style>&#x05D0;&#x05D1;&#x05D2;ABC.</div>
      </div>
      <div dir="rtl">
        <div dir="rtl"><style>body {color:black;}</style>&#x05D0;&#x05D1;&#x05D2;ABC.</div>
      </div>
    </div>
  </body>
</html>
```

```json
{
  "messages": [
    {
      "category": "Html",
      "code": "html.style.not_allowed_here",
      "message": "Element “style” not allowed as child of “div” in this context.",
      "severity": "Warning",
      "span": {
        "byte_end": 1557,
        "byte_start": 1550,
        "col": 25,
        "line": 43
      }
    },
    {
      "category": "Html",
      "code": "html.style.not_allowed_here",
      "message": "Element “style” not allowed as child of “div” in this context.",
      "severity": "Warning",
      "span": {
        "byte_end": 1685,
        "byte_start": 1678,
        "col": 25,
        "line": 46
      }
    },
    {
      "category": "Html",
      "code": "html.style.not_allowed_here",
      "message": "Element “style” not allowed as child of “div” in this context.",
      "severity": "Warning",
      "span": {
        "byte_end": 1845,
        "byte_start": 1838,
        "col": 24,
        "line": 51
      }
    },
    {
      "category": "Html",
      "code": "html.style.not_allowed_here",
      "message": "Element “style” not allowed as child of “div” in this context.",
      "severity": "Warning",
      "span": {
        "byte_end": 1972,
        "byte_start": 1965,
        "col": 24,
        "line": 54
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
  "source_name": "html/dom/elements/global-attributes/dir_auto-contained-style-R.html"
}
```
