# html/semantics/embedded-content/the-iframe-element/sandbox_008-manual.htm

Counts:
- errors: 0
- warnings: 3
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/embedded-content/the-iframe-element/sandbox_008-manual.htm",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<html>
<head>
    <title>HTML5 Sandbox: Allow sandboxed iframe content to navigate the sandboxed browsing context itself.</title>
    <meta content="text/html; charset=UTF-8" http-equiv="Content-Type" />
    <link rel="author" title="Microsoft" href="http://www.microsoft.com/" />
    <link rel="help" href="http://dev.w3.org/html5/spec/Overview.html#sandboxed-navigation-browsing-context-flag" />
    <meta name="assert" content="Allow sandboxed iframe content to navigate the sandboxed browsing context itself." />
    <script src="support/sandbox_helper.js" type="text/javascript"></script>
</head>
<body>
    <pre>Description: Allow sandboxed iframe content to navigate the sandboxed browsing context itself.</pre>
    <table id='testtable' border='1'>
        <tr>
            <td>Test Result</td>
            <td>Test Assertion</td>
        </tr>
        <tr>
            <td id='test_0_result'>Manual</td>
            <td id='test_0_assertion'>
                <div>Steps:</div>
                <div>1. Click link "Click here to perform self navigation".</div>
                <br />
                <div>Test passes if there is no red on the page and the word "PASS" appears in the below iframe after following the above steps.</div>
            </td>
        </tr>
    </table>
    <br />
    <div id="testframe">
        <pre>iframe with sandbox=""</pre>
        <iframe id="iframe1" name="iframe1" src="support/iframe_sandbox_008.htm" sandbox="" style="height: 100px; width: 350px;"></iframe>
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
        "byte_end": 600,
        "byte_start": 537,
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
        "byte_end": 1566,
        "byte_start": 1535,
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
  "source_name": "html/semantics/embedded-content/the-iframe-element/sandbox_008-manual.htm"
}
```
