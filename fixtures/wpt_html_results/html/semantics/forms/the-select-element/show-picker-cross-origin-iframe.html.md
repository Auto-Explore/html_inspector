# html/semantics/forms/the-select-element/show-picker-cross-origin-iframe.html

Counts:
- errors: 3
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/forms/the-select-element/show-picker-cross-origin-iframe.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<title>Test showPicker() called from cross-origin iframe</title>
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<script src="/common/get-host-info.sub.js"></script>
<body>
<iframe id="iframe1"></iframe>
<iframe id="iframe2"></iframe>
<iframe id="iframe3"></iframe>
<iframe id="iframe4"></iframe>
</body>
<script>
    function waitForSecurityErrors() {
        return new Promise((resolve) => {
            window.addEventListener("message", (event) => resolve(event.data), {
                once: true,
            });
        });
    }

    promise_test(async (t) => {
        iframe1.src =
            new URL("resources/", self.location).pathname +
            "show-picker-child-iframe.html";

        // Wait for the iframe to report security errors when calling showPicker().
        const securityErrors = await waitForSecurityErrors();
        assert_equals(
            securityErrors,
            "",
            "In same-origin iframes, showPicker() does not throw a SecurityError."
        );
    });

    promise_test(async (t) => {
        iframe2.src =
            get_host_info().HTTP_NOTSAMESITE_ORIGIN +
            new URL("resources/", self.location).pathname +
            "show-picker-child-iframe.html";

        // Wait for the iframe to report security errors when calling showPicker().
        const securityErrors = await waitForSecurityErrors();
        assert_equals(
            securityErrors,
            "select",
            "In cross-origin iframes, showPicker() throws a SecurityError."
        );
    });

    promise_test(async (t) => {
        iframe3.src =
            new URL("resources/", self.location).pathname +
            "show-picker-child-iframe.html?documentDomain=" + get_host_info().ORIGINAL_HOST;

        // Wait for the iframe to report security errors when calling showPicker().
        const securityErrors = await waitForSecurityErrors();
        assert_equals(
            securityErrors,
            "",
            "In same-origin but cross-origin-domain iframes, showPicker() does not throw a SecurityError."
        );
    });

    promise_test(async (t) => {
        document.domain = get_host_info().ORIGINAL_HOST;
        iframe4.src =
            get_host_info().HTTP_REMOTE_ORIGIN +
            new URL("resources/", self.location).pathname +
            "show-picker-child-iframe.html?documentDomain=" + get_host_info().ORIGINAL_HOST;

        // Wait for the iframe to report security errors when calling showPicker().
        const securityErrors = await waitForSecurityErrors();
        assert_equals(
            securityErrors,
            "select",
            "In cross-origin but same-origin-domain iframes, showPicker() throws a SecurityError."
        );
    });
</script>
```

```json
{
  "messages": [
    {
      "category": "Html",
      "code": "html.parser.stray_start_tag",
      "message": "Stray start tag “script”.",
      "severity": "Error",
      "span": {
        "byte_end": 387,
        "byte_start": 379,
        "col": 1,
        "line": 12
      }
    },
    {
      "category": "Html",
      "code": "html.parser.nonspace_after_body",
      "message": "Non-space character after body.",
      "severity": "Error",
      "span": {
        "byte_end": 2817,
        "byte_start": 387,
        "col": 9,
        "line": 12
      }
    },
    {
      "category": "Html",
      "code": "html.parser.end_tag_after_body_closed",
      "message": "Saw an end tag after “body” had been closed.",
      "severity": "Error",
      "span": {
        "byte_end": 2826,
        "byte_start": 2817,
        "col": 1,
        "line": 79
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
  "source_name": "html/semantics/forms/the-select-element/show-picker-cross-origin-iframe.html"
}
```
