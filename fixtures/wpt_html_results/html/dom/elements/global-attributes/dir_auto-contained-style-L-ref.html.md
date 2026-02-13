# html/dom/elements/global-attributes/dir_auto-contained-style-L-ref.html

Counts:
- errors: 0
- warnings: 5
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/dom/elements/global-attributes/dir_auto-contained-style-L-ref.html",
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
    <title>HTML Test: dir=auto, start with style, then L</title>
    <link rel="author" title="Matitiahu Allouche" href="mailto:matitiahu.allouche@google.com" />
    <link rel="author" title="Oren Roth" href="mailto:oren.roth@gmail.com" />
    <link rel="author" title="HTML5 bidi test WG" href="mailto:html5bidi@googlegroups.com" />
    <link rel="help" href="https://html.spec.whatwg.org/multipage/#the-dir-attribute" />
    <meta name="assert" content="
      When dir='auto', the direction is set according to the first strong character
      of descendant text while ignoring descendant style elements.
      In this test, it is the Latin letter A, thus the direction must be
      resolved as LTR." />
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
        <div dir="ltr"><style>body {color:black;}</style>ABC&#x05D0;&#x05D1;&#x05D2;.</div>
      </div>
      <div dir="rtl">
        <div dir="ltr"><style>body {color:black;}</style>ABC&#x05D0;&#x05D1;&#x05D2;.</div>
      </div>
    </div>
    <div class="ref">
      <div dir="ltr">
        <div dir="ltr"><style>body {color:black;}</style>ABC&#x05D0;&#x05D1;&#x05D2;.</div>
      </div>
      <div dir="rtl">
        <div dir="ltr"><style>body {color:black;}</style>ABC&#x05D0;&#x05D1;&#x05D2;.</div>
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
        "byte_end": 1484,
        "byte_start": 1477,
        "col": 24,
        "line": 42
      }
    },
    {
      "category": "Html",
      "code": "html.style.not_allowed_here",
      "message": "Element “style” not allowed as child of “div” in this context.",
      "severity": "Warning",
      "span": {
        "byte_end": 1611,
        "byte_start": 1604,
        "col": 24,
        "line": 45
      }
    },
    {
      "category": "Html",
      "code": "html.style.not_allowed_here",
      "message": "Element “style” not allowed as child of “div” in this context.",
      "severity": "Warning",
      "span": {
        "byte_end": 1771,
        "byte_start": 1764,
        "col": 24,
        "line": 50
      }
    },
    {
      "category": "Html",
      "code": "html.style.not_allowed_here",
      "message": "Element “style” not allowed as child of “div” in this context.",
      "severity": "Warning",
      "span": {
        "byte_end": 1898,
        "byte_start": 1891,
        "col": 24,
        "line": 53
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
  "source_name": "html/dom/elements/global-attributes/dir_auto-contained-style-L-ref.html"
}
```
