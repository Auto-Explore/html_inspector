# html/browsers/browsing-the-web/overlapping-navigations-and-traversals/anchor-fragment-history-back-on-click.html

Counts:
- errors: 0
- warnings: 2
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/browsers/browsing-the-web/overlapping-navigations-and-traversals/anchor-fragment-history-back-on-click.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<script>
promise_test(async t => {
  // Wait for after the load event so that the navigation doesn't get converted
  // into a replace navigation.
  await new Promise(resolve => window.onload = () => t.step_timeout(resolve, 0));

  location.hash = "#1";
  assert_equals(location.hash, "#1");
  location.hash = "#2";
  assert_equals(location.hash, "#2");

  let anchor = document.createElement("a");
  anchor.href = "#3";
  anchor.onclick = () => {
    history.back();
  };

  let navigations = [];
  let navigationsPromise = new Promise(resolve => {
    onpopstate = () => {
      navigations.push(location.hash);
      if (navigations.length === 2) {
        resolve();
      }
    }
  });

  anchor.click();
  await navigationsPromise;

  // We were on #2 when history.back() was called so we should be on #1 now.
  assert_equals(location.hash, "#1");

  // While the history navigation back to "#1" was pending, we should have navigated to "#3".
  assert_array_equals(navigations, ["#3", "#1"]);
}, "Anchor with a fragment href and a click handler that navigates back");
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
  "source_name": "html/browsers/browsing-the-web/overlapping-navigations-and-traversals/anchor-fragment-history-back-on-click.html"
}
```
