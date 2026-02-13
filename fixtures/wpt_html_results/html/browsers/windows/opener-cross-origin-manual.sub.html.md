# html/browsers/windows/opener-cross-origin-manual.sub.html

Counts:
- errors: 3
- warnings: 2
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/browsers/windows/opener-cross-origin-manual.sub.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<ol>
 <li><p>Clicking this link must navigate this page to a resource that contains "THE END":
 <a href=//{{domains[www1]}}:{{location[port]}}/html/browsers/windows/resources/opener-cross-origin.html target=_blank>test</a>

 <li><p>Clicking this link must open a new browsing context that is empty:
 <a rel=noreferrer href=//{{domains[www1]}}:{{location[port]}}/html/browsers/windows/resources/opener-cross-origin.html target=_blank>test</a>

 <li><p>Clicking this link must navigate this page to a resource that contains "THE END":
 <a href=//{{domains[www1]}}:{{location[port]}}/html/browsers/windows/resources/opener-cross-origin-embed.sub.html target=_blank>test</a>
</ol>
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
        "byte_end": 4,
        "byte_start": 0,
        "col": 1,
        "line": 1
      }
    },
    {
      "category": "Html",
      "code": "html.parse.p.end_tag_implied_open_elements",
      "message": "End tag “p” implied, but there were open elements.",
      "severity": "Error",
      "span": {
        "byte_end": 232,
        "byte_start": 229,
        "col": 6,
        "line": 5
      }
    },
    {
      "category": "Html",
      "code": "html.parse.p.end_tag_implied_open_elements",
      "message": "End tag “p” implied, but there were open elements.",
      "severity": "Error",
      "span": {
        "byte_end": 451,
        "byte_start": 448,
        "col": 6,
        "line": 8
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
  "source_name": "html/browsers/windows/opener-cross-origin-manual.sub.html"
}
```
