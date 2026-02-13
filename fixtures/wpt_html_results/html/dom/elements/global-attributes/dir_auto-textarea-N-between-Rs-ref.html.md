# html/dom/elements/global-attributes/dir_auto-textarea-N-between-Rs-ref.html

Counts:
- errors: 0
- warnings: 2
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/dom/elements/global-attributes/dir_auto-textarea-N-between-Rs-ref.html",
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
    <link rel="author" title="Aharon Lanin" href="mailto:aharon@google.com" />
    <link rel="author" title="HTML5 bidi test WG" href="mailto:html5bidi@googlegroups.com" />
    <style>
      body, textarea {
        font-size:18px;
        text-align:left;
      }
      textarea {
        resize: none;
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
    </div>
    <div class="test">
      <div dir="ltr">
        <textarea rows="4" dir="rtl">
&#x05D0;
!...
&#x05D0;
        </textarea>
      </div>
      <div dir="rtl">
        <textarea rows="4" dir="rtl">
&#x05D0;
!...
&#x05D0;
        </textarea>
      </div>
    </div>
    <div class="ref">
      <div dir="ltr">
        <textarea rows="4" dir="rtl">
&#x05D0;
!...
&#x05D0;
        </textarea>
      </div>
      <div dir="rtl">
        <textarea rows="4" dir="rtl">
&#x05D0;
!...
&#x05D0;
        </textarea>
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
  "source_name": "html/dom/elements/global-attributes/dir_auto-textarea-N-between-Rs-ref.html"
}
```
