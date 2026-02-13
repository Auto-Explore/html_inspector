# html/semantics/embedded-content/the-iframe-element/support/iframe_sandbox_021.htm

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/embedded-content/the-iframe-element/support/iframe_sandbox_021.htm",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<html>
<head>
    <title>Page with iframes</title>
</head>
<body>
    <table cellpadding="5" cellspacing="10">
        <tr>
            <td>
                <span>child iframe with sandbox="allow-scripts" attribute</span><br />
                <iframe id="Iframe1" src="iframe_sandbox_021a.htm" sandbox="allow-scripts" style="height: 50px; width: 250px;"></iframe>
            </td>
        </tr>
        <tr>
            <td>
                <span>child iframe with sandbox="" attribute</span><br />
                <iframe id="Iframe2" src="iframe_sandbox_020a.htm" sandbox="" style="height: 50px; width: 250px;"></iframe>
            </td>
        </tr>
        <tr>
            <td>
                <span>child iframe without sandbox attribute</span><br />
                <iframe id="Iframe3" src="iframe_sandbox_021a.htm" style="height: 50px; width: 250px;"></iframe>
            </td>
        </tr>
    </table>
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
  "source_name": "html/semantics/embedded-content/the-iframe-element/support/iframe_sandbox_021.htm"
}
```
