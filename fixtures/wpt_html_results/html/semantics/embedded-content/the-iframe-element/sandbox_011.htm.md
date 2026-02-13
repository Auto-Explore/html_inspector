# html/semantics/embedded-content/the-iframe-element/sandbox_011.htm

Counts:
- errors: 0
- warnings: 3
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/embedded-content/the-iframe-element/sandbox_011.htm",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<html>
<head>
    <title>HTML5 Sandbox: iframe sandbox attribute value support DOMTokenList interface.</title>
    <meta content="text/html; charset=UTF-8" http-equiv="Content-Type" />
    <link rel="author" title="Microsoft" href="http://www.microsoft.com/" />
    <link rel="help" href="http://dev.w3.org/html5/spec/Overview.html#the-iframe-element" />
    <meta name="assert" content="iframe sandbox attribute value support DOMTokenList interface." />
    <script src="/resources/testharness.js"></script>
    <script src="/resources/testharnessreport.js"></script>
</head>
<body>
    <div id=log></div>
    <iframe id="iframe1" src="about:blank" sandbox="allow-scripts allow-same-origin allow-forms" style="display : none"></iframe>
    <script type="text/javascript">

    test(function() {
        var iframeEle = document.getElementById("iframe1");
        assert_equals(iframeEle.sandbox.length, 3)
    }, "DOMTokenList length")

    test(function() {
        var iframeEle = document.getElementById("iframe1");
        assert_equals(iframeEle.sandbox.item(1),  "allow-same-origin")
    }, "DOMTokenList item(index)")

    test(function() {
        var iframeEle = document.getElementById("iframe1");
        assert_true(iframeEle.sandbox.contains("allow-forms"))
    }, "DOMTokenList contains(DomString)")

    test(function() {
        var iframeEle = document.getElementById("iframe1");
        iframeEle.sandbox.add("ALLOW-SANDBOX");
        assert_true(iframeEle.sandbox.contains("ALLOW-SANDBOX"))
    }, "DOMTokenList add(DomString)")

    test(function() {
        var iframeEle = document.getElementById("iframe1");
        iframeEle.sandbox.remove("ALLOW-SANDBOX");
        assert_false(iframeEle.sandbox.contains("ALLOW-SANDBOX"))
    }, "DOMTokenList remove(DomString)")

    test(function() {
        var iframeEle = document.getElementById("iframe1");
        iframeEle.sandbox.remove("ALLOW-SANDBOX");
        assert_true(
            iframeEle.sandbox.toggle("allow-top-navigation") && iframeEle.sandbox.contains("allow-top-navigation") &&
            !iframeEle.sandbox.toggle("allow-top-navigation") && !iframeEle.sandbox.contains("allow-top-navigation")
        )
    }, "DOMTokenList toggle(DomString) - Returns true if token is now present (it was added); returns false if it is not (it was removed).")

    test(function() {
        var iframeEle = document.getElementById("iframe1");
        assert_equals(iframeEle.sandbox.value, iframeEle.sandbox.toString())
    }, "DOMTokenList sandbox.toString()")

    test(function() {
        var iframeEle = document.getElementById("iframe1");
        iframeEle.sandbox.remove("ALLOW-SANDBOX");
        assert_true(iframeEle.sandbox.contains("allow-scripts") != iframeEle.sandbox.contains("Allow-SCRIPTS"))
    }, "DOMTokenList case sensitivity")
    </script>
</body>
</html>
```

```json
{
  "messages": [
    {
      "category": "Html",
      "code": "html.iframe.sandbox.scripts_and_same_origin",
      "message": "Bad value “allow-scripts allow-same-origin allow-forms” for attribute “sandbox” on element “iframe”.",
      "severity": "Warning",
      "span": {
        "byte_end": 743,
        "byte_start": 627,
        "col": 5,
        "line": 14
      }
    },
    {
      "category": "Html",
      "code": "html.script.type.unnecessary",
      "message": "The “type” attribute is unnecessary for JavaScript resources.",
      "severity": "Warning",
      "span": {
        "byte_end": 788,
        "byte_start": 757,
        "col": 5,
        "line": 15
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
  "source_name": "html/semantics/embedded-content/the-iframe-element/sandbox_011.htm"
}
```
