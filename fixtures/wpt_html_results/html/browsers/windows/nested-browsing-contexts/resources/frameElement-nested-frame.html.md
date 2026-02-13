# html/browsers/windows/nested-browsing-contexts/resources/frameElement-nested-frame.html

Counts:
- errors: 0
- warnings: 2
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/browsers/windows/nested-browsing-contexts/resources/frameElement-nested-frame.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<meta charset="utf-8"/>
<title>HTML Test: child browsing context created by the frame element</title>
<link rel="author" title="Intel" href="http://www.intel.com/" />
<frameset>
  <frame id="f1" name="frame">
</frameset>
```

```json
{
  "messages": [
    {
      "category": "Html",
      "code": "html.element.frameset.obsolete",
      "message": "The “frameset” element is obsolete. Use the “iframe” element and CSS instead, or use server-side includes.",
      "severity": "Warning",
      "span": {
        "byte_end": 193,
        "byte_start": 183,
        "col": 1,
        "line": 5
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
  "source_name": "html/browsers/windows/nested-browsing-contexts/resources/frameElement-nested-frame.html"
}
```
