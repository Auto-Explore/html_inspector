# html/semantics/embedded-content/the-iframe-element/srcdoc_change_hash.html

Counts:
- errors: 1
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/embedded-content/the-iframe-element/srcdoc_change_hash.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<title>same-document navigation inside an srcdoc iframe using location.hash</title>
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<script src="/resources/testdriver.js"></script>
<script src="/resources/testdriver-vendor.js"></script>
<script>
  promise_test(async () => {
    // Wait until 'document' is available.
    await new Promise(resolve => window.addEventListener('load', resolve));

    // Create an iframe, wait until is is loaded.
    let iframe = document.createElement('iframe');
    await new Promise(resolve => {
      iframe.srcdoc = "srcdoc document";
      iframe.onload = resolve;
      document.body.appendChild(iframe);
    });

    assert_equals(iframe.contentDocument.body.innerText, "srcdoc document");
    assert_equals(iframe.contentWindow.location.href, "about:srcdoc");

    function iframeHashChanged() {
      return new Promise(resolve => {
        iframe.contentWindow.onhashchange = resolve;
      })
    }

    // 1) hash = "1".
    {
      let hash_changed = iframeHashChanged();
      await test_driver.bless("hash = '1'", () => {
        iframe.contentWindow.location.hash = "1";
      });
      await hash_changed;
      assert_equals(iframe.contentWindow.location.href, "about:srcdoc#1");
    }

    // 2) hash = "2".
    {
      let hash_changed = iframeHashChanged();
      await test_driver.bless("hash = '2'", () => {
        iframe.contentWindow.location.hash = "2";
      });
      await hash_changed;
      assert_equals(iframe.contentWindow.location.href, "about:srcdoc#2");
    }

    // 3) history.back().
    {
      let hash_changed = iframeHashChanged();
      await test_driver.bless("history.back()", () => {
        history.back();
      });
      await hash_changed;
      assert_equals(iframe.contentWindow.location.href, "about:srcdoc#1");
    }

    // 4) history.forward().
    {
      let hash_changed = iframeHashChanged();
      await test_driver.bless("history.forward()", () => {
        history.forward();
      });
      await hash_changed;
      assert_equals(iframe.contentWindow.location.href, "about:srcdoc#2");
    }
  });
</script>
```

```json
{
  "messages": [
    {
      "category": "Html",
      "code": "html.parser.start_tag_without_doctype",
      "message": "Start tag seen without seeing a doctype first. Expected “<!DOCTYPE html>”.",
      "severity": "Error",
      "span": {
        "byte_end": 7,
        "byte_start": 0,
        "col": 1,
        "line": 1
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
  "source_name": "html/semantics/embedded-content/the-iframe-element/srcdoc_change_hash.html"
}
```
