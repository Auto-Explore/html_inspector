# html/browsers/windows/browsing-context-names/duplicate-name-order.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/browsers/windows/browsing-context-names/duplicate-name-order.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<title>Duplicate name lookup order</title>
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<script src="/common/utils.js"></script>
<script src="/common/dispatcher/dispatcher.js"></script>
<script src="/html/browsers/browsing-the-web/remote-context-helper/resources/remote-context-helper.js"></script>

<body>
<script>

promise_test(async t => {
  const rcHelper = new RemoteContextHelper();
  const rcMain = await rcHelper.addWindow();

  const rcPopupA = await rcHelper.addWindow(undefined, { target: 'A' });
  const rcPopupB = await rcHelper.addWindow(undefined, { target: 'B' });
  const rcPopupC = await rcHelper.addWindow(undefined, { target: 'C' });

  const rcSiblingA = await rcMain.addIframe(undefined, { name: 'A' });
  const rcSiblingB = await rcMain.addIframe(undefined, { name: 'B' });

  const rcRequestor = await rcMain.addIframe();

  const rcChildA = await rcRequestor.addIframe(undefined, { name: 'A' });

  const windowIdentifiers = {
   'Main': rcMain,
   'PopupA': rcPopupA,
   'PopupB': rcPopupB,
   'PopupC': rcPopupC,
   'SiblingA': rcSiblingA,
   'SiblingB': rcSiblingB,
   'Requestor': rcRequestor,
   'ChildA': rcChildA,
  };
  for (const [windowIdentifier, remote] of Object.entries(windowIdentifiers)) {
    await remote.executeScript((windowIdentifier) => {
      window.windowIdentifier = windowIdentifier;
    }, [windowIdentifier]);
  }

  function performLookup(targetName) {
    return rcRequestor.executeScript((targetName) => {
        return window.open('', targetName).windowIdentifier;
      }, [targetName]);
  }

  assert_equals(await performLookup('A'), 'ChildA', 'subtree first');
  assert_equals(await performLookup('B'), 'SiblingB', 'then the rest of the tree');
  assert_equals(await performLookup('C'), 'PopupC', 'then other pages');
}, 'Duplicate name lookup order');

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
  "source_name": "html/browsers/windows/browsing-context-names/duplicate-name-order.html"
}
```
