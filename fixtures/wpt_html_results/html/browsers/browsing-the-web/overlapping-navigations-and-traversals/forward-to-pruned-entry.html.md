# html/browsers/browsing-the-web/overlapping-navigations-and-traversals/forward-to-pruned-entry.html

Counts:
- errors: 0
- warnings: 2
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/browsers/browsing-the-web/overlapping-navigations-and-traversals/forward-to-pruned-entry.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!doctype html>
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<script>
promise_test(async t => {
  // Wait for after the load event so that the navigation doesn't get converted
  // into a replace navigation.
  await new Promise(resolve => window.onload = () => t.step_timeout(resolve, 0));
  location.hash = "#1";
  location.hash = "#2";
  history.go(-2);
  await new Promise(r => window.onpopstate = r);

  // Traverse forward then immediately do a same-document push. This will
  // truncate the back forward list.
  history.forward();
  location.hash = "#clobber";

  // history.forward() should be aborted.
  window.onpopstate = t.unreached_func("history.forward() should have been cancelled");
  await new Promise(r => t.step_timeout(r, 20));
  assert_equals(location.hash, "#clobber");
}, "If forward pruning clobbers the target of a traverse, abort");
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
  "source_name": "html/browsers/browsing-the-web/overlapping-navigations-and-traversals/forward-to-pruned-entry.html"
}
```
