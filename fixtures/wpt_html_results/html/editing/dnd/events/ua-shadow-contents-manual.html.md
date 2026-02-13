# html/editing/dnd/events/ua-shadow-contents-manual.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/editing/dnd/events/ua-shadow-contents-manual.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<html lang="en">
<meta charset="utf-8">
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>

<body draggable="true" style="padding:20px">
Drag this input down quickly. Try multiple times. No errors should be reported.<br>
<input type=color>

<script>
document.body.addEventListener('dragenter', (e) => {
  let target = e.relatedTarget;
  while (target) {
    assert_false(target instanceof ShadowRoot,'Drag events should not expose UA shadow roots');
    // This console log can also cause DevTools crashes, so this is explicitly
    // left in on purpose:
    console.log(target);
    target = target.parentNode;
  }
});
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
    }
  ],
  "source_name": "html/editing/dnd/events/ua-shadow-contents-manual.html"
}
```
