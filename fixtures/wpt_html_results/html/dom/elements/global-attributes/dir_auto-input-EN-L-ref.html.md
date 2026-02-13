# html/dom/elements/global-attributes/dir_auto-input-EN-L-ref.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/dom/elements/global-attributes/dir_auto-input-EN-L-ref.html",
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
    <title>HTML Test: input with dir=auto, start with EN+L</title>
    <link rel="author" title="Matitiahu Allouche" href="mailto:matitiahu.allouche@google.com" />
    <link rel="author" title="Oren Roth" href="mailto:oren.roth@gmail.com" />
    <link rel="author" title="Shai Berger" href="mailto:shai@platonix.com" />
    <link rel="author" title="HTML5 bidi test WG" href="mailto:html5bidi@googlegroups.com" />
    <link rel="help" href="https://html.spec.whatwg.org/multipage/#the-dir-attribute" />
    <meta name="assert" content="
      When dir='auto', the direction of an input element is set according to
      the first strong character of its value.
      In this test, it is the Latin letter A since digits are not strongly
      directional, thus the direction must be resolved as LTR." />
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
        <input type="text" dir="ltr" value="123ABC&#x05D0;&#x05D1;&#x05D2;." />
      </div>
      <div dir="rtl">
        <input type="text" dir="ltr" value="123ABC&#x05D0;&#x05D1;&#x05D2;." />
      </div>
    </div>
    <div class="ref">
      <div dir="ltr">
        <input type="text" dir="ltr" value="123ABC&#x05D0;&#x05D1;&#x05D2;." />
      </div>
      <div dir="rtl">
        <input type="text" dir="ltr" value="123ABC&#x05D0;&#x05D1;&#x05D2;." />
      </div>
    </div>
  </body>
</html>
```

```json
{
  "messages": [
    {
      "category": "I18n",
      "code": "i18n.lang.missing",
      "message": "Consider adding a “lang” attribute to the “html” start tag to declare the language of this document.",
      "severity": "Warning",
      "span": null
    }
  ],
  "source_name": "html/dom/elements/global-attributes/dir_auto-input-EN-L-ref.html"
}
```
