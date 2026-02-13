# html/semantics/popovers/input-type-popovertarget.html

Counts:
- errors: 0
- warnings: 5
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/popovers/input-type-popovertarget.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<link rel=author href="mailto:jarhar@chromium.org">
<link rel=author href="mailto:lwarlow@igalia.com">
<link rel=help href="https://issues.chromium.org/issues/329118508">
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>

<div id=mypopover popover=auto>popover</div>

<iframe name=foo></iframe>
<form id="form" target=foo action="about:blank">
  <input id=reset-in-form type=reset popovertarget=mypopover value="reset">
  <input id=submit-in-form type=submit popovertarget=mypopover value="submit">
  <input id=button-in-form type=button popovertarget=mypopover value="type=button">
  <input id=image-in-form type=image popovertarget=mypopover value="type=image">
</form>

<input id=reset-attr-form type=reset popovertarget=mypopover value="reset" form=form>
<input id=submit-attr-form type=submit popovertarget=mypopover value="submit" form=form>
<input id=button-attr-form type=button popovertarget=mypopover value="type=button" form=form>
<input id=image-attr-form type=image popovertarget=mypopover value="type=image" form=form>

<input id=reset-outside-form type=reset popovertarget=mypopover value="reset">
<input id=submit-outside-form type=submit popovertarget=mypopover value="submit">
<input id=button-outside-form type=button popovertarget=mypopover value="type=button">
<input id=image-outside-form type=image popovertarget=mypopover value="type=image">

<script>
test((t) => {
  let formReset = false;
  function onReset(e) {
    e.preventDefault();
    formReset = true;
  }
  form.addEventListener('reset', onReset);
  t.add_cleanup(() => {
    mypopover.hidePopover();
    form.removeEventListener('reset', onReset);
  });
  document.getElementById('reset-in-form').click();
  assert_true(formReset, 'type=reset should trigger form reset event');
  assert_false(mypopover.matches(':popover-open'), 'type=reset should not toggle the popover');
}, 'input type=reset in form should trigger form reset and not toggle popover');

test((t) => {
  let formSubmit = false;
  function onSubmit(e) {
    e.preventDefault();
    formSubmit = true;
  }
  form.addEventListener('submit', onSubmit);
  t.add_cleanup(() => {
    mypopover.hidePopover();
    form.removeEventListener('submit', onSubmit);
  });
  document.getElementById('submit-in-form').click();
  assert_true(formSubmit, 'type=submit should trigger form submit event');
  assert_false(mypopover.matches(':popover-open'), 'type=submit should not toggle the popover');
}, 'input type=submit in form should trigger form submit and not toggle popover');

test((t) => {
  t.add_cleanup(() => {
    mypopover.hidePopover();
  });
  document.getElementById('button-in-form').click();
  assert_true(mypopover.matches(':popover-open'), 'type=button should toggle the popover');
}, 'input type=button in form should toggle popover');

test((t) => {
  let formSubmit = false;
  function onSubmit(e) {
    e.preventDefault();
    formSubmit = true;
  }
  form.addEventListener('submit', onSubmit);
  t.add_cleanup(() => {
    mypopover.hidePopover();
    form.removeEventListener('submit', onSubmit);
  });
  document.getElementById('image-in-form').click();
  assert_true(formSubmit, 'type=image should trigger form submit event');
  assert_false(mypopover.matches(':popover-open'), 'type=image should not toggle the popover');
}, 'input type=image in form should trigger form submit and not toggle popover');

test((t) => {
  let formReset = false;
  function onReset(e) {
    e.preventDefault();
    formReset = true;
  }
  form.addEventListener('reset', onReset);
  t.add_cleanup(() => {
    mypopover.hidePopover();
    form.removeEventListener('reset', onReset);
  });
  document.getElementById('reset-attr-form').click();
  assert_true(formReset, 'type=reset should trigger form reset event');
  assert_false(mypopover.matches(':popover-open'), 'type=reset should not toggle the popover');
}, 'input type=reset with form attr should trigger form reset and not toggle popover');

test((t) => {
  let formSubmit = false;
  function onSubmit(e) {
    e.preventDefault();
    formSubmit = true;
  }
  form.addEventListener('submit', onSubmit);
  t.add_cleanup(() => {
    mypopover.hidePopover();
    form.removeEventListener('submit', onSubmit);
  });
  document.getElementById('submit-attr-form').click();
  assert_true(formSubmit, 'type=submit should trigger form submit event');
  assert_false(mypopover.matches(':popover-open'), 'type=submit should not toggle the popover');
}, 'input type=submit with form attr should trigger form submit and not toggle popover');

test((t) => {
  t.add_cleanup(() => {
    mypopover.hidePopover();
  });
  document.getElementById('button-attr-form').click();
  assert_true(mypopover.matches(':popover-open'), 'type=button should toggle the popover');
}, 'input type=button with form attr should toggle popover');

test((t) => {
  let formSubmit = false;
  function onSubmit(e) {
    e.preventDefault();
    formSubmit = true;
  }
  form.addEventListener('submit', onSubmit);
  t.add_cleanup(() => {
    mypopover.hidePopover();
    form.removeEventListener('submit', onSubmit);
  });
  document.getElementById('image-attr-form').click();
  assert_true(formSubmit, 'type=image should trigger form submit event');
  assert_false(mypopover.matches(':popover-open'), 'type=image should not toggle the popover');
}, 'input type=image with form attr should trigger form submit and not toggle popover');

test((t) => {
  t.add_cleanup(() => {
    mypopover.hidePopover();
  });
  document.getElementById('reset-outside-form').click();
  assert_true(mypopover.matches(':popover-open'), 'type=reset should toggle the popover');
}, 'input type=reset outside form should toggle popover');

test((t) => {
  t.add_cleanup(() => {
    mypopover.hidePopover();
  });
  document.getElementById('submit-outside-form').click();
  assert_true(mypopover.matches(':popover-open'), 'type=submit should toggle the popover');
}, 'input type=submit outside form should toggle popover');

test((t) => {
  t.add_cleanup(() => {
    mypopover.hidePopover();
  });
  document.getElementById('button-outside-form').click();
  assert_true(mypopover.matches(':popover-open'), 'type=button should toggle the popover');
}, 'input type=button outside form should toggle popover');

test((t) => {
  t.add_cleanup(() => {
    mypopover.hidePopover();
  });
  document.getElementById('image-outside-form').click();
  assert_true(mypopover.matches(':popover-open'), 'type=image should toggle the popover');
}, 'input type=image outside form should toggle popover');
</script>
```

```json
{
  "messages": [
    {
      "category": "Html",
      "code": "html.input.image.alt.missing",
      "message": "Element “input” is missing required attribute “alt”.",
      "severity": "Warning",
      "span": {
        "byte_end": 735,
        "byte_start": 657,
        "col": 3,
        "line": 15
      }
    },
    {
      "category": "Html",
      "code": "html.input.image.alt.missing",
      "message": "Element “input” is missing required attribute “alt”.",
      "severity": "Warning",
      "span": {
        "byte_end": 1104,
        "byte_start": 1014,
        "col": 1,
        "line": 21
      }
    },
    {
      "category": "Html",
      "code": "html.input.image.alt.missing",
      "message": "Element “input” is missing required attribute “alt”.",
      "severity": "Warning",
      "span": {
        "byte_end": 1437,
        "byte_start": 1354,
        "col": 1,
        "line": 26
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
  "source_name": "html/semantics/popovers/input-type-popovertarget.html"
}
```
