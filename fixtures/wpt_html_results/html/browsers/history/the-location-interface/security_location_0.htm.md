# html/browsers/history/the-location-interface/security_location_0.htm

Counts:
- errors: 0
- warnings: 2
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/browsers/history/the-location-interface/security_location_0.htm",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<html>
  <head>
    <title>Location interface Security</title>
    <link rel="author" title="Microsoft" href="http://www.microsoft.com/" />
    <link rel="help" href="https://html.spec.whatwg.org/multipage/#security-location" />
    <meta name="assert" content="access location object from different origins doesn't raise SECURITY_ERR exception" />
    <script src="/resources/testharness.js"></script>
    <script src="/resources/testharnessreport.js"></script>
  </head>
  <body>
    <p>Access location object from different origins doesn't raise SECURITY_ERR exception</p>
    <div id=log></div>
    <script src="/common/get-host-info.sub.js"></script>
    <script>
      var runTest = async_test("Accessing location object from different origins doesn't raise SECURITY_ERR exception").step_func_done(function() {
        var frame = document.getElementById('testframe');
        frame.setAttribute('onload', '');
        frame.contentWindow.location = get_host_info().HTTP_REMOTE_ORIGIN + "/";
      });
    </script>
    <iframe id='testframe' onload="runTest()">Test Frame</iframe>
    <script>
      document.getElementById('testframe').setAttribute('src', get_host_info().HTTP_REMOTE_ORIGIN + '/');
    </script>
  </body>
</html>
```

```json
{
  "messages": [
    {
      "category": "Html",
      "code": "html.iframe.text.disallowed",
      "message": "Text not allowed in “iframe” in this context.",
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
  "source_name": "html/browsers/history/the-location-interface/security_location_0.htm"
}
```
