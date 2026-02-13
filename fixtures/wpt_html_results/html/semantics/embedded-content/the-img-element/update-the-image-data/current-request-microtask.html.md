# html/semantics/embedded-content/the-img-element/update-the-image-data/current-request-microtask.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/embedded-content/the-img-element/update-the-image-data/current-request-microtask.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<title>An img's current request should be updated in a microtask after selecting an image source</title>
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<div id=log></div>

<script>
async_test(function(t) {
  const picture = document.createElement("picture");

  const nonMatchingSource = document.createElement("source");
  nonMatchingSource.media = "not all";
  nonMatchingSource.srcset = "data:,a";
  picture.append(nonMatchingSource);

  const matchingSource = document.createElement("source");
  matchingSource.media = "all";
  matchingSource.srcset = "data:,b";
  picture.append(matchingSource);

  const img = document.createElement("img");
  img.src = "data:,c";

  assert_equals(img.currentSrc, "", "after assigning to img.src but before the corresponding microtask is run");

  queueMicrotask(t.step_func(function() {
    assert_equals(img.currentSrc, "data:,c", "after assigning to img.src and after corresponding microtask is run");

    picture.append(img);
    assert_equals(img.currentSrc, "data:,c", "after appending img to picture but before the corresponding microtask is run");

    queueMicrotask(t.step_func(function() {
      assert_equals(img.currentSrc, "data:,b", "after appending img to picture and after the corresponding microtask is run");
      t.done();
    }));
  }));
}, "currentSrc is updated only after the microtask that updates the current request is run");
</script>
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
  "source_name": "html/semantics/embedded-content/the-img-element/update-the-image-data/current-request-microtask.html"
}
```
