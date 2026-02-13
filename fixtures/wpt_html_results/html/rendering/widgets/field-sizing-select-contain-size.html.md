# html/rendering/widgets/field-sizing-select-contain-size.html

Counts:
- errors: 0
- warnings: 3
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/rendering/widgets/field-sizing-select-contain-size.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<link rel="help" href="https://drafts.csswg.org/css-ui-4/#field-sizing">
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<body>
<style>
.disable-default {
  field-sizing: content;
}
.contain {
    contain: size;
}
</style>
<div id="container"></div>
<script>
const container = document.querySelector('#container');
const DISABLE = 'class="disable-default"';

// Tests for drop-down box ====================================================

test(() => {
  const s = '<select class="contain">><option>1<option>quick brown<option>fox</select>';
  container.innerHTML = s + s;
  container.lastElementChild.style.fieldSizing = 'content';
  const widthForContent1 = container.lastElementChild.offsetWidth;
  assert_greater_than_equal(container.firstElementChild.offsetWidth,
                      widthForContent1);
  container.lastElementChild.selectedIndex = 1;
  const widthForContentQuickBrown = container.lastElementChild.offsetWidth;
  assert_equals(widthForContentQuickBrown, widthForContent1);
}, 'dropdown: The width should not depend on the selected OPTION when contain:size is set');

// Tests for list box =========================================================

// Some platforms don't support list box rendering.
container.innerHTML = '<select></select><select multiple></select>';
if (container.firstElementChild.offsetHeight != container.lastElementChild.offsetHeight) {
  test(() => {
    container.innerHTML = `<select class="contain" multiple><option>fox</select>` +
                          `<select class="contain disable-default" multiple><option>fox</select>`;
    const former = container.firstElementChild;
    const latter = container.lastElementChild;
    const widthForOneItem = latter.offsetWidth;
    assert_equals(former.offsetWidth, widthForOneItem);

    latter.add(new Option('quick brown'));
    assert_equals(latter.offsetWidth, widthForOneItem);
  }, 'listbox: The width should not depend on content when contain:size is set');
}
</script>
</body>
```

```json
{
  "messages": [
    {
      "category": "Html",
      "code": "html.style.not_allowed_here",
      "message": "Element “style” not allowed as child of “body” in this context.",
      "severity": "Warning",
      "span": {
        "byte_end": 209,
        "byte_start": 202,
        "col": 1,
        "line": 6
      }
    },
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
  "source_name": "html/rendering/widgets/field-sizing-select-contain-size.html"
}
```
