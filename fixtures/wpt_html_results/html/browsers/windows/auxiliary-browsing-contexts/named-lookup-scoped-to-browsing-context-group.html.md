# html/browsers/windows/auxiliary-browsing-contexts/named-lookup-scoped-to-browsing-context-group.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/browsers/windows/auxiliary-browsing-contexts/named-lookup-scoped-to-browsing-context-group.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<title>Named lookup scoped to browsing context group</title>
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<script src="/common/utils.js"></script>
<script src="/common/dispatcher/dispatcher.js"></script>
<script src="/html/browsers/browsing-the-web/remote-context-helper/resources/remote-context-helper.js"></script>

<body>
<script>

promise_test(async t => {
  const rcHelper = new RemoteContextHelper();
  // This test harness cannot be in the same browsing context group as the
  // popup.
  const rcPopup1 = await rcHelper.addWindow(undefined, { features: 'noopener' });

  await rcPopup1.executeScript(() => {
    window.name = 'my_popup';
    window.firstPopup = true;
  });
  assert_equals(window.open('', 'my_popup').firstPopup, undefined,
                'cannot lookup across browsing context groups');
}, 'named lookup scoped to browsing context group');

</script>
</body>
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
  "source_name": "html/browsers/windows/auxiliary-browsing-contexts/named-lookup-scoped-to-browsing-context-group.html"
}
```
