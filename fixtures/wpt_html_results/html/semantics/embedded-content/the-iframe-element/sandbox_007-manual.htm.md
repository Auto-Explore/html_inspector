# html/semantics/embedded-content/the-iframe-element/sandbox_007-manual.htm

Counts:
- errors: 0
- warnings: 4
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/embedded-content/the-iframe-element/sandbox_007-manual.htm",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE HTML>
<html>
<head>
    <title>HTML5 Sandbox: Block form submission inside sandbox iframe</title>
    <meta content="text/html; charset=UTF-8" http-equiv="Content-Type" />
    <link rel="author" title="Microsoft" href="http://www.microsoft.com/" />
    <link rel="help" href="http://dev.w3.org/html5/spec/Overview.html#sandboxed-forms-browsing-context-flag" />
    <meta name="assert" content="Block form submission inside sandbox iframe." />
    <script src="support/sandbox_helper.js" type="text/javascript"></script>
</head>
<body>
    <pre>Description: Block form submission inside iframe with sandbox attribute.</pre>
    <table id='testtable' border='1'>
        <tr>
            <td>Test Result</td>
            <td>Test Assertion</td>
        </tr>
        <tr>
            <td id='test_0_result'>Manual</td>
            <td id='test_0_assertion'>
                <div>Steps:</div>
                <div>1. Click button "Submit Form".</div>
                <br />
                <div>Test passes if there is no red on the page and there is no navigation in the below iframe after following the above steps.</div>
            </td>
        </tr>
    </table>
    <br />
    <div id="testframe">
        <pre>iframe with sandbox="allow-scripts allow-same-origin allow-top-navigation"</pre>
        <iframe src="support/iframe_sandbox_007.htm" sandbox="allow-scripts allow-same-origin allow-top-navigation" style="height: 100px; width: 300px;"></iframe>
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
        "byte_end": 520,
        "byte_start": 457,
        "col": 5,
        "line": 9
      }
    },
    {
      "category": "Html",
      "code": "html.iframe.sandbox.scripts_and_same_origin",
      "message": "Bad value “allow-scripts allow-same-origin allow-top-navigation” for attribute “sandbox” on element “iframe”.",
      "severity": "Warning",
      "span": {
        "byte_end": 1459,
        "byte_start": 1314,
        "col": 9,
        "line": 31
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
        "line": 33
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
  "source_name": "html/semantics/embedded-content/the-iframe-element/sandbox_007-manual.htm"
}
```
