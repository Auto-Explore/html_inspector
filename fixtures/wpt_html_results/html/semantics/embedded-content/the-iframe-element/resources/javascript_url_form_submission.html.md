# html/semantics/embedded-content/the-iframe-element/resources/javascript_url_form_submission.html

Counts:
- errors: 1
- warnings: 4
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/embedded-content/the-iframe-element/resources/javascript_url_form_submission.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<img src="/images/blue.png">
<form action="javascript:(() => { parent.javascriptUrlRan = true; })()"><button>submit</button><form>
```

```json
{
  "messages": [
    {
      "category": "Html",
      "code": "html.img.alt.missing",
      "message": "An “img” element must have an “alt” attribute, except under certain conditions. For details, consult guidance on providing text alternatives for images.",
      "severity": "Warning",
      "span": {
        "byte_end": 44,
        "byte_start": 16,
        "col": 1,
        "line": 2
      }
    },
    {
      "category": "Html",
      "code": "html.form.action.invalid",
      "message": "Bad value “javascript:(() => { parent.javascriptUrlRan = true; })()” for attribute “action” on element “form”.",
      "severity": "Warning",
      "span": {
        "byte_end": 117,
        "byte_start": 45,
        "col": 1,
        "line": 3
      }
    },
    {
      "category": "Html",
      "code": "html.parser.nested_form",
      "message": "Saw a “form” start tag, but there was already an active “form” element. Nested forms are not allowed. Ignoring the tag.",
      "severity": "Error",
      "span": {
        "byte_end": 146,
        "byte_start": 140,
        "col": 96,
        "line": 3
      }
    },
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
  "source_name": "html/semantics/embedded-content/the-iframe-element/resources/javascript_url_form_submission.html"
}
```
