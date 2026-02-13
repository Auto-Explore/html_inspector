# html/browsers/the-window-object/accessing-other-browsing-contexts/test1.html

Counts:
- errors: 0
- warnings: 2
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/browsers/the-window-object/accessing-other-browsing-contexts/test1.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<meta charset="utf-8">
<title>HTML Test: child browsing contexts created by iframe elements</title>
<link rel="author" title="Intel" href="http://www.intel.com/" />
<table id="tbl">
  <tr>
    <td>
      <iframe id="fr4" src=""></iframe>
    </td>
  </tr>
  <iframe id="fr5" src="about:blank"></iframe>
</table>
```

```json
{
  "messages": [
    {
      "category": "Html",
      "code": "html.iframe.src.empty",
      "message": "Bad value “” for attribute “src” on element “iframe”.",
      "severity": "Warning",
      "span": {
        "byte_end": 309,
        "byte_start": 274,
        "col": 3,
        "line": 11
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
  "source_name": "html/browsers/the-window-object/accessing-other-browsing-contexts/test1.html"
}
```
