# html/browsers/windows/targeting-multiple-cross-origin-manual.sub.html

Counts:
- errors: 1
- warnings: 2
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/browsers/windows/targeting-multiple-cross-origin-manual.sub.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<meta charset=utf-8>
<p>Follow this link to open a new browsing context in a separate origin. Follow the instructions
in that new window, and then come back to this window.
<a target=first href="{{location[scheme]}}://{{domains[天気の良い日]}}:{{location[port]}}/html/browsers/windows/resources/target-cross-origin.sub.html">link</a>.

<p>Once you come back to this page, follow this link.
<a target=second href="resources/echo-window-name.html">link</a>.

<p>After clicking that link, you should have three additional windows open.
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
        "byte_end": 20,
        "byte_start": 0,
        "col": 1,
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
  "source_name": "html/browsers/windows/targeting-multiple-cross-origin-manual.sub.html"
}
```
