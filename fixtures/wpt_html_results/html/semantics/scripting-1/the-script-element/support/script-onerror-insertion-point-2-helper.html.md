# html/semantics/scripting-1/the-script-element/support/script-onerror-insertion-point-2-helper.html

Counts:
- errors: 1
- warnings: 3
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/scripting-1/the-script-element/support/script-onerror-insertion-point-2-helper.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
Some <script src="http://this is not parseable:-80/"
             onerror="document.write('text'); document.close(); parent.writeDone(document.documentElement.textContent)"></script>
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
        "byte_end": 173,
        "byte_start": 5,
        "col": 6,
        "line": 1
      }
    },
    {
      "category": "Html",
      "code": "html.script.src.invalid",
      "message": "Bad value “http://this is not parseable:-80/” for attribute “src” on element “script”.",
      "severity": "Warning",
      "span": {
        "byte_end": 173,
        "byte_start": 5,
        "col": 6,
        "line": 1
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
  "source_name": "html/semantics/scripting-1/the-script-element/support/script-onerror-insertion-point-2-helper.html"
}
```
