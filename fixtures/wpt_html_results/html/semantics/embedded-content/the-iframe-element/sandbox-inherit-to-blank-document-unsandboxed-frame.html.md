# html/semantics/embedded-content/the-iframe-element/sandbox-inherit-to-blank-document-unsandboxed-frame.html

Counts:
- errors: 0
- warnings: 2
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/embedded-content/the-iframe-element/sandbox-inherit-to-blank-document-unsandboxed-frame.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<html>
<head>
  <script src="/resources/testharness.js"></script>
  <script src="/resources/testharnessreport.js"></script>
</head>
<body>
<script>
  // Sandbox flags are inherited from a document toward every frame it creates,
  // which then is inherited to every new document created in this frame.
  //
  // Using the flag 'allow-popups-to-escape-sandbox' inhibits this inheritance
  // mechanism when the new frame is a popup.
  //
  // Sandbox flags are not inherited from the initiator/creator when loading a
  // local scheme document unlike CSP (tested in
  // ./sandbox-inherit-to-blank-document-unsandboxed.html)
  //
  // This tests in particular the initial empty document and the first
  // about:blank navigation and verifies that no sandbox is applied on the
  // popups.
  promise_test(async test => {
    const msg = await new Promise(r => window.addEventListener("message", r));
    assert_false(msg.data.access_initial_navigation_to_about_blank_throws,
      "Failed to access initial about:blank popup, it is probably sandboxed"
    );
    assert_false(msg.data.access_first_navigation_to_about_blank_throws,
      "Failed to access navigation to about:blank, it is probably sandboxed"
    );
    assert_false(msg.data.access_after_delay_initial_navigation_to_about_blank_throws,
      "Failed to access navigation to about:blank, it is probably sandboxed"
    );
    assert_false(msg.data.access_after_delay_first_navigation_to_about_blank_throws,
      "Failed to access navigation to about:blank, it is probably sandboxed"
    );
  }, "Popup do not inherit sandbox, because of " +
     "'allow-popups-to-escape-sandbox'. The document isn't sandboxed.")

</script>
<iframe
  sandbox="allow-scripts allow-popups allow-popups-to-escape-sandbox"
  srcdoc="
  <script>
    let access_initial_navigation_to_about_blank_throws = false;
    let access_first_navigation_to_about_blank_throws = false;
    let access_after_delay_initial_navigation_to_about_blank_throws = false;
    let access_after_delay_first_navigation_to_about_blank_throws = false;
    const initial_about_blank_window =
      window.open('/common/blank.html?pipe=status(204)');
    try {
      initial_about_blank_window.origin;
    } catch(e) {
      access_initial_navigation_to_about_blank_throws = true;
    }
    const renavigated_about_blank_window = window.open('about:blank');
    try {
      renavigated_about_blank_window.origin;
    } catch(e) {
      access_first_navigation_to_about_blank_throws = true;
    }
    setTimeout(() => {
      try {
        initial_about_blank_window.origin;
      } catch(e) {
        access_after_delay_initial_navigation_to_about_blank_throws = true;
      }
      try {
        renavigated_about_blank_window.origin;
      } catch(e) {
        access_after_delay_first_navigation_to_about_blank_throws = true;
      }
      top.postMessage({
        'access_initial_navigation_to_about_blank_throws':
          access_initial_navigation_to_about_blank_throws,
        'access_first_navigation_to_about_blank_throws':
          access_first_navigation_to_about_blank_throws,
        'access_after_delay_initial_navigation_to_about_blank_throws':
          access_after_delay_initial_navigation_to_about_blank_throws,
        'access_after_delay_first_navigation_to_about_blank_throws':
          access_after_delay_first_navigation_to_about_blank_throws
      }, '*');
    }, 500);
  </script>"
>
</iframe>
</body>
</html>
```

```json
{
  "messages": [
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
  "source_name": "html/semantics/embedded-content/the-iframe-element/sandbox-inherit-to-blank-document-unsandboxed-frame.html"
}
```
