# html/semantics/embedded-content/the-iframe-element/sandbox_020-manual.htm

Counts:
- errors: 0
- warnings: 3
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/embedded-content/the-iframe-element/sandbox_020-manual.htm",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<html>
<head>
    <title>HTML5 Sandbox: Nested iframes cannot have less sandbox restrictions than their most restrictive ancestor iframe.</title>
    <meta content="text/html; charset=UTF-8" http-equiv="Content-Type" />
    <link rel="author" title="Microsoft" href="http://www.microsoft.com/" />
    <link rel="help" href="http://dev.w3.org/html5/spec/Overview.html#attr-iframe-sandbox" />
    <meta name="assert" content="Nested iframes cannot have less sandbox restrictions than their most restrictive ancestor iframe." />
    <script src="support/sandbox_helper.js" type="text/javascript"></script>
</head>
<body>
    <pre>Description: Nested iframes cannot have less sandbox restrictions than their most restrictive ancestor iframe.</pre>
    <div>This test is to verify script is blocked inside nested iframes if the top-most sandbox iframe has no 'allow-scripts' token.</div>
    <br />
    <table id='testtable' border='1'>
        <tr>
            <td>Test Result</td>
            <td>Test Assertion</td>
        </tr>
        <tr>
            <td id='test_0_result'>Manual</td>
            <td id='test_0_assertion'>Test passes if there is no red on the page.</td>
        </tr>
    </table>
    <br />
    <div id="testframe">
        <div style="font-weight:bold">Top-most iframe with sandbox=""</div>
        <iframe id="iframe1" name="iframe1" src="support/iframe_sandbox_020.htm" sandbox="" style="height: 330px; width: 400px;"></iframe>
    </div>
    <script type="text/javascript">
        DisableTestForNonSupportingBrowsers();
    </script>
</body>
</html>
```

```json
{
  "messages": [
    {
      "category": "Html",
      "code": "html.script.type.unnecessary",
      "message": "The “type” attribute is unnecessary for JavaScript resources.",
      "severity": "Warning",
      "span": {
        "byte_end": 609,
        "byte_start": 546,
        "col": 5,
        "line": 9
      }
    },
    {
      "category": "Html",
      "code": "html.script.type.unnecessary",
      "message": "The “type” attribute is unnecessary for JavaScript resources.",
      "severity": "Warning",
      "span": {
        "byte_end": 1515,
        "byte_start": 1484,
        "col": 5,
        "line": 30
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
  "source_name": "html/semantics/embedded-content/the-iframe-element/sandbox_020-manual.htm"
}
```
