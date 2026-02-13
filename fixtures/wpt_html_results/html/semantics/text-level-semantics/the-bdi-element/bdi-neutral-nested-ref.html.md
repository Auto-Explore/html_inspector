# html/semantics/text-level-semantics/the-bdi-element/bdi-neutral-nested-ref.html

Counts:
- errors: 0
- warnings: 2
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/text-level-semantics/the-bdi-element/bdi-neutral-nested-ref.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<html>
  <head>
    <meta charset="utf-8">
    <link rel="author" title="Aharon Lanin" href="mailto:aharon@google.com">
    <link rel="author" title="HTML5 bidi test WG" href="mailto:html5bidi@googlegroups.com">
    <style>
      body{
        font-size:2em;
      }
      .test, .ref {
        border: medium solid gray;
        width: 500px;
        margin: 20px;
      }
      .comments { display: none; }
    </style>
  </head>
  <body>
    <div class="instructions"><p>Test passes if the two boxes below look exactly the same.</p></div>
    <div class="comments">
      Key to entities used below:
        &#x05D0; ... &#x05D5; - The first six Hebrew letters (strongly RTL).
        &#x202D; - The LRO (left-to-right override) formatting character.
        &#x202C; - The PDF (pop directional formatting) formatting character; closes LRO.
    </div>
    <div class="test">
      <div dir="ltr">&#x202D;1 + [a + [3 + [b + 4] + &#x05D1;] + 2] + &#x05D0;&#x202C;</div>
      <div dir="ltr">&#x202D;1 + [a + [3 + [b + 4] + &#x05D1;] + 2] + &#x05D0;&#x202C;</div>
      <div dir="ltr">&#x202D;1 + [a + [3 + [b + 4] + &#x05D1;] + 2] + &#x05D0;&#x202C;</div>
      <div dir="rtl">&#x202D;a + [1 + [b + [3 + &#x05D1;] + 2] + &#x05D0;] + 0&#x202C;</div>
      <div dir="rtl">&#x202D;a + [1 + [b + [3 + &#x05D1;] + 2] + &#x05D0;] + 0&#x202C;</div>
      <div dir="rtl">&#x202D;a + [1 + [b + [3 + &#x05D1;] + 2] + &#x05D0;] + 0&#x202C;</div>
    </div>
    <div class="ref">
      <div dir="ltr">&#x202D;1 + [a + [3 + [b + 4] + &#x05D1;] + 2] + &#x05D0;&#x202C;</div>
      <div dir="ltr">&#x202D;1 + [a + [3 + [b + 4] + &#x05D1;] + 2] + &#x05D0;&#x202C;</div>
      <div dir="ltr">&#x202D;1 + [a + [3 + [b + 4] + &#x05D1;] + 2] + &#x05D0;&#x202C;</div>
      <div dir="rtl">&#x202D;a + [1 + [b + [3 + &#x05D1;] + 2] + &#x05D0;] + 0&#x202C;</div>
      <div dir="rtl">&#x202D;a + [1 + [b + [3 + &#x05D1;] + 2] + &#x05D0;] + 0&#x202C;</div>
      <div dir="rtl">&#x202D;a + [1 + [b + [3 + &#x05D1;] + 2] + &#x05D0;] + 0&#x202C;</div>
    </div>
  </body>
</html>
```

```json
{
  "messages": [
    {
      "category": "Html",
      "code": "html.head.title.missing",
      "message": "Element “head” is missing a required instance of child element “title”.",
      "severity": "Warning",
      "span": null
    },
    {
      "category": "I18n",
      "code": "i18n.lang.missing",
      "message": "Consider adding a “lang” attribute to the “html” start tag to declare the language of this document.",
      "severity": "Warning",
      "span": null
    }
  ],
  "source_name": "html/semantics/text-level-semantics/the-bdi-element/bdi-neutral-nested-ref.html"
}
```
