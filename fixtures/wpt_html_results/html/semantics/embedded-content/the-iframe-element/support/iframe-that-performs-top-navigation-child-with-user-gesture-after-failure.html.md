# html/semantics/embedded-content/the-iframe-element/support/iframe-that-performs-top-navigation-child-with-user-gesture-after-failure.html

Counts:
- errors: 0
- warnings: 2
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/embedded-content/the-iframe-element/support/iframe-that-performs-top-navigation-child-with-user-gesture-after-failure.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!doctype html>

<button>Gain user activation</button>

<script src="/resources/testdriver.js"></script>
<script src="/resources/testdriver-vendor.js"></script>
<script>
  test_driver.set_test_context(top.opener);
  window.onload = async function () {
    try {
      top.location = "navigation-changed-iframe.html";
      return;
    } catch (e) {
      top.opener.postMessage("ERROR", "*");
    }

    const button = document.querySelector("button");
    await test_driver.click(button);

    try {
      top.location = "navigation-changed-iframe.html";
    } catch (e) {
      top.opener.postMessage("ERROR", "*");
    }
  };
</script>
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
  "source_name": "html/semantics/embedded-content/the-iframe-element/support/iframe-that-performs-top-navigation-child-with-user-gesture-after-failure.html"
}
```
