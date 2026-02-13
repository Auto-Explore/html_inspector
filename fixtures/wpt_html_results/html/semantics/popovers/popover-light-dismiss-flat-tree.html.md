# html/semantics/popovers/popover-light-dismiss-flat-tree.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/popovers/popover-light-dismiss-flat-tree.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<html>
<head>
    <meta charset="utf-8">
    <title>Test that popover light dismiss uses the flat tree.</title>
    <link rel="author" title="Tim Nguyen" href="https://github.com/nt1m">
</head>
<body>
    <p>Test passes if the inner popover opens after clicking the inner toggle.</p>
    <button popovertarget="outerPopover" popovertargetaction="toggle" id="outerPopoverToggle">Toggle</button>
    <div id="outerPopover" popover>
        <template shadowrootmode="open">
            Outer
            <button popovertarget="innerPopover" popovertargetaction="toggle" id="innerPopoverToggle">Toggle</button>
            <div id="innerPopover" popover>
                Inner
            </div>
        </template>
    </div>
    <script src="/resources/testharness.js"></script>
    <script src="/resources/testharnessreport.js"></script>
    <script src="/resources/testdriver.js"></script>
    <script src="/resources/testdriver-actions.js"></script>
    <script src="/resources/testdriver-vendor.js"></script>
    <script src="resources/popover-utils.js"></script>
    <script>
        promise_test(async () => {
            const innerPopoverToggle = outerPopover.shadowRoot.querySelector("#innerPopoverToggle");
            const innerPopover = outerPopover.shadowRoot.querySelector("#innerPopover");

            assert_false(outerPopover.matches(":popover-open"), "outer popover is initially hidden");
            assert_false(innerPopover.matches(":popover-open"), "inner popover is initially hidden");

            await clickOn(outerPopoverToggle);

            assert_true(outerPopover.matches(":popover-open"), "outer popover is open after clicking the toggle");
            assert_false(innerPopover.matches(":popover-open"), "inner popover is initially hidden");

            await clickOn(innerPopoverToggle);

            assert_true(outerPopover.matches(":popover-open"), "outer popover is not dismissed after clicking the second toggle");
            assert_true(innerPopover.matches(":popover-open"), "inner popover is open after clicking the second toggle");
        }, "Popover light dismiss uses the flat tree");
    </script>
</body>
</html>
```

```json
{
  "messages": [
    {
      "category": "I18n",
      "code": "i18n.lang.missing",
      "message": "Consider adding a “lang” attribute to the “html” start tag to declare the language of this document.",
      "severity": "Warning",
      "span": null
    }
  ],
  "source_name": "html/semantics/popovers/popover-light-dismiss-flat-tree.html"
}
```
