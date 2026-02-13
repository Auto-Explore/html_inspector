# html/browsers/windows/noreferrer-cross-origin-manual.html

Counts:
- errors: 2
- warnings: 2
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/browsers/windows/noreferrer-cross-origin-manual.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<ol>
 <li><p>After clicking these two links in order a single browsing context should be open showing
 <code>example.org</code>:
 <a target=doesnotmatter href="http://example.com/">one</a>,
 <a target=doesnotmatter href="http://example.org/">two</a>.

 <li><p>After clicking these two links two browsing contexts should have been opened:
 <a rel=noreferrer target=reallydoesnotmatter href="http://example.com/">one</a>,
 <a rel=noreferrer target=reallydoesnotmatter href="http://example.com/">two</a>.
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
        "byte_end": 260,
        "byte_start": 257,
        "col": 6,
        "line": 7
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
  "source_name": "html/browsers/windows/noreferrer-cross-origin-manual.html"
}
```
