# html/semantics/embedded-content/the-iframe-element/iframe-loading-lazy-in-scroller-nested-5.html

Counts:
- errors: 0
- warnings: 2
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/embedded-content/the-iframe-element/iframe-loading-lazy-in-scroller-nested-5.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<link rel="help" href="https://html.spec.whatwg.org/#lazy-load-root-margin">
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>

<style>
  #scroller {
    width: 100px;
    height: 100px;
    overflow: scroll;
    background-color: gray;
    display: flex;
  }

  #scroller2 {
    width: 110px;
    height: 110px;
    overflow: scroll;
  }

  .spacer {
    width: 130px;
    height: 130px;
    flex-shrink: 0;
  }

  #target {
    width: 100px;
    height: 100px;
    flex-shrink: 0;
  }
</style>

<div id=scroller2>
  <div class="spacer"></div>
  <div id="scroller">
    <div class="spacer"></div>
    <iframe
      id="target"
      src="resources/subframe.html"
      loading="lazy"
      onload="iframe_onload();"
    ></iframe>
  </div>
</div>

<script>
  const t = async_test(
    "Test that lazy-loaded images load when near viewport."
  );

  function iframe_onload() {
    t.done();
  }

  t.step_timeout(() => {
    t.unreached_func(
      "Timed out while waiting for iframe to load."
    )();
  }, 2000);
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
  "source_name": "html/semantics/embedded-content/the-iframe-element/iframe-loading-lazy-in-scroller-nested-5.html"
}
```
