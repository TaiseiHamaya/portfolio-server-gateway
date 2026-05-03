const _: () = ::protobuf::__internal::assert_compatible_gencode_version("4.32.1-release");
// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut Proto__PayloadPlayerZoneEnterBegin_msg_init: ::protobuf::__internal::runtime::MiniTablePtr =
    ::protobuf::__internal::runtime::MiniTablePtr(::std::ptr::null_mut());
#[allow(non_camel_case_types)]
pub struct PayloadPlayerZoneEnterBegin {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<PayloadPlayerZoneEnterBegin>
}

impl ::protobuf::Message for PayloadPlayerZoneEnterBegin {}

impl ::std::default::Default for PayloadPlayerZoneEnterBegin {
  fn default() -> Self {
    Self::new()
  }
}

impl ::protobuf::Parse for PayloadPlayerZoneEnterBegin {
  fn parse(serialized: &[u8]) -> ::std::result::Result<Self, ::protobuf::ParseError> {
    Self::parse(serialized)
  }

  fn parse_dont_enforce_required(serialized: &[u8]) -> ::std::result::Result<Self, ::protobuf::ParseError> {
    Self::parse_dont_enforce_required(serialized)
  }
}

impl ::std::fmt::Debug for PayloadPlayerZoneEnterBegin {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    let string = unsafe {
      ::protobuf::__internal::runtime::debug_string(
        self.raw_msg(),
        <Self as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table()
      )
    };
    write!(f, "{}", string)
  }
}

impl ::protobuf::Serialize for PayloadPlayerZoneEnterBegin {
  fn serialize(&self) -> ::std::result::Result<Vec<u8>, ::protobuf::SerializeError> {
    ::protobuf::AsView::as_view(self).serialize()
  }
}

// SAFETY:
// - `PayloadPlayerZoneEnterBegin` is `Sync` because it does not implement interior mutability.
//    Neither does `PayloadPlayerZoneEnterBeginMut`.
unsafe impl Sync for PayloadPlayerZoneEnterBegin {}

// SAFETY:
// - `PayloadPlayerZoneEnterBegin` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl Send for PayloadPlayerZoneEnterBegin {}

impl ::protobuf::Proxied for PayloadPlayerZoneEnterBegin {
  type View<'msg> = PayloadPlayerZoneEnterBeginView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for PayloadPlayerZoneEnterBegin {}

impl ::protobuf::MutProxied for PayloadPlayerZoneEnterBegin {
  type Mut<'msg> = PayloadPlayerZoneEnterBeginMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct PayloadPlayerZoneEnterBeginView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, PayloadPlayerZoneEnterBegin>,
  _phantom: ::std::marker::PhantomData<&'msg ()>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for PayloadPlayerZoneEnterBeginView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for PayloadPlayerZoneEnterBeginView<'msg> {
  type Message = PayloadPlayerZoneEnterBegin;
}

impl ::std::fmt::Debug for PayloadPlayerZoneEnterBeginView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    let string = unsafe {
      ::protobuf::__internal::runtime::debug_string(
        self.raw_msg(),
        <Self as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table()
      )
    };
    write!(f, "{}", string)
  }
}

impl ::protobuf::Serialize for PayloadPlayerZoneEnterBeginView<'_> {
  fn serialize(&self) -> ::std::result::Result<Vec<u8>, ::protobuf::SerializeError> {
    // SAFETY: `MINI_TABLE` is the one associated with `self.raw_msg()`.
    let encoded = unsafe {
      ::protobuf::__internal::runtime::wire::encode(self.raw_msg(),
          <Self as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table())
    };
    encoded.map_err(|_| ::protobuf::SerializeError)
  }
}

impl ::std::default::Default for PayloadPlayerZoneEnterBeginView<'_> {
  fn default() -> PayloadPlayerZoneEnterBeginView<'static> {
    let inner = unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap_raw(::protobuf::__internal::runtime::ScratchSpace::zeroed_block()) };
    PayloadPlayerZoneEnterBeginView::new(::protobuf::__internal::Private, inner)
  }
}

#[allow(dead_code)]
impl<'msg> PayloadPlayerZoneEnterBeginView<'msg> {
  #[doc(hidden)]
  pub fn new(_private: ::protobuf::__internal::Private, inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, PayloadPlayerZoneEnterBegin>) -> Self {
    Self { inner, _phantom: ::std::marker::PhantomData }
  }

  fn raw_msg(&self) -> ::protobuf::__internal::runtime::RawMessage {
    self.inner.raw()
  }

  pub fn to_owned(&self) -> PayloadPlayerZoneEnterBegin {
    ::protobuf::IntoProxied::into_proxied(*self, ::protobuf::__internal::Private)
  }

  // user_id: optional uint64
  pub fn user_id(self) -> u64 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_u64_at_index(
        0, (0u64).into()
      ).try_into().unwrap()
    }
  }

  // gateway_id: optional uint64
  pub fn gateway_id(self) -> u64 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_u64_at_index(
        1, (0u64).into()
      ).try_into().unwrap()
    }
  }

}

// SAFETY:
// - `PayloadPlayerZoneEnterBeginView` is `Sync` because it does not support mutation.
unsafe impl Sync for PayloadPlayerZoneEnterBeginView<'_> {}

// SAFETY:
// - `PayloadPlayerZoneEnterBeginView` is `Send` because while its alive a `PayloadPlayerZoneEnterBeginMut` cannot.
// - `PayloadPlayerZoneEnterBeginView` does not use thread-local data.
unsafe impl Send for PayloadPlayerZoneEnterBeginView<'_> {}

impl<'msg> ::protobuf::Proxy<'msg> for PayloadPlayerZoneEnterBeginView<'msg> {}
impl<'msg> ::protobuf::ViewProxy<'msg> for PayloadPlayerZoneEnterBeginView<'msg> {}

impl<'msg> ::protobuf::AsView for PayloadPlayerZoneEnterBeginView<'msg> {
  type Proxied = PayloadPlayerZoneEnterBegin;
  fn as_view(&self) -> ::protobuf::View<'msg, PayloadPlayerZoneEnterBegin> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for PayloadPlayerZoneEnterBeginView<'msg> {
  fn into_view<'shorter>(self) -> PayloadPlayerZoneEnterBeginView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<PayloadPlayerZoneEnterBegin> for PayloadPlayerZoneEnterBeginView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> PayloadPlayerZoneEnterBegin {
    let mut dst = PayloadPlayerZoneEnterBegin::new();
    let dst_raw = ::protobuf::__internal::runtime::UpbGetMessagePtrMut::get_raw_message_mut(&mut dst, ::protobuf::__internal::Private);
    let dst_arena = ::protobuf::__internal::runtime::UpbGetArena::get_arena(&mut dst, ::protobuf::__internal::Private);
    let src_raw = ::protobuf::__internal::runtime::UpbGetMessagePtr::get_raw_message(&self, ::protobuf::__internal::Private);

    unsafe { ::protobuf::__internal::runtime::upb_Message_DeepCopy(
      dst_raw,
      src_raw,
      <Self as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
      dst_arena.raw(),
    ) };
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<PayloadPlayerZoneEnterBegin> for PayloadPlayerZoneEnterBeginMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> PayloadPlayerZoneEnterBegin {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::runtime::UpbTypeConversions for PayloadPlayerZoneEnterBegin {
    fn upb_type() -> ::protobuf::__internal::runtime::CType {
        ::protobuf::__internal::runtime::CType::Message
    }

    fn to_message_value(
        val: ::protobuf::View<'_, Self>) -> ::protobuf::__internal::runtime::upb_MessageValue {
        ::protobuf::__internal::runtime::upb_MessageValue { msg_val: Some(val.raw_msg()) }
    }

    unsafe fn into_message_value_fuse_if_required(
      raw_parent_arena: ::protobuf::__internal::runtime::RawArena,
      mut val: Self) -> ::protobuf::__internal::runtime::upb_MessageValue {
      // SAFETY: The arena memory is not freed due to `ManuallyDrop`.
      let parent_arena = ::std::mem::ManuallyDrop::new(
          unsafe { ::protobuf::__internal::runtime::Arena::from_raw(raw_parent_arena) });

      parent_arena.fuse(val.as_message_mut_inner(::protobuf::__internal::Private).arena());
      ::protobuf::__internal::runtime::upb_MessageValue { msg_val: Some(val.raw_msg()) }
    }

    unsafe fn from_message_value<'msg>(msg: ::protobuf::__internal::runtime::upb_MessageValue)
        -> ::protobuf::View<'msg, Self> {
        let raw = unsafe { msg.msg_val }.expect("expected present message value in map");
        let inner = unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap_raw(raw) };
        PayloadPlayerZoneEnterBeginView::new(::protobuf::__internal::Private, inner)
    }

    unsafe fn from_message_mut<'msg>(msg: ::protobuf::__internal::runtime::RawMessage, arena: &'msg ::protobuf::__internal::runtime::Arena)
        -> PayloadPlayerZoneEnterBeginMut<'msg> {
        let inner = unsafe { ::protobuf::__internal::runtime::MessageMutInner::<'msg, PayloadPlayerZoneEnterBegin>::wrap_raw(msg, arena) };
        PayloadPlayerZoneEnterBeginMut::new(::protobuf::__internal::Private, inner)
    }
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct PayloadPlayerZoneEnterBeginMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, PayloadPlayerZoneEnterBegin>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for PayloadPlayerZoneEnterBeginMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for PayloadPlayerZoneEnterBeginMut<'msg> {
  type Message = PayloadPlayerZoneEnterBegin;
}

impl ::std::fmt::Debug for PayloadPlayerZoneEnterBeginMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    let string = unsafe {
      ::protobuf::__internal::runtime::debug_string(
        self.raw_msg(),
        <Self as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table()
      )
    };
    write!(f, "{}", string)
  }
}

impl ::protobuf::Serialize for PayloadPlayerZoneEnterBeginMut<'_> {
  fn serialize(&self) -> ::std::result::Result<Vec<u8>, ::protobuf::SerializeError> {
    ::protobuf::AsView::as_view(self).serialize()
  }
}

#[allow(dead_code)]
impl<'msg> PayloadPlayerZoneEnterBeginMut<'msg> {
  #[doc(hidden)]
  pub fn from_parent<ParentT: ::protobuf::Message>(
             _private: ::protobuf::__internal::Private,
             parent: ::protobuf::__internal::runtime::MessageMutInner<'msg, ParentT>,
             msg: ::protobuf::__internal::runtime::RawMessage)
    -> Self {
    Self {
      inner: ::protobuf::__internal::runtime::MessageMutInner::from_parent(parent, msg)
    }
  }

  #[doc(hidden)]
  pub fn new(_private: ::protobuf::__internal::Private, inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, PayloadPlayerZoneEnterBegin>) -> Self {
    Self { inner }
  }

  fn raw_msg(&self) -> ::protobuf::__internal::runtime::RawMessage {
    self.inner.raw()
  }

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, PayloadPlayerZoneEnterBegin> {
    self.inner
  }

  pub fn to_owned(&self) -> PayloadPlayerZoneEnterBegin {
    ::protobuf::AsView::as_view(self).to_owned()
  }

  fn arena(&mut self) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }

  // user_id: optional uint64
  pub fn user_id(&self) -> u64 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_u64_at_index(
        0, (0u64).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_user_id(&mut self, val: u64) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_u64_at_index(
        0, val.into()
      )
    }
  }

  // gateway_id: optional uint64
  pub fn gateway_id(&self) -> u64 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_u64_at_index(
        1, (0u64).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_gateway_id(&mut self, val: u64) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_u64_at_index(
        1, val.into()
      )
    }
  }

}

// SAFETY:
// - `PayloadPlayerZoneEnterBeginMut` does not perform any shared mutation.
// - `PayloadPlayerZoneEnterBeginMut` is not `Send`, and so even in the presence of mutator
//   splitting, synchronous access of an arena is impossible.
unsafe impl Sync for PayloadPlayerZoneEnterBeginMut<'_> {}

impl<'msg> ::protobuf::Proxy<'msg> for PayloadPlayerZoneEnterBeginMut<'msg> {}
impl<'msg> ::protobuf::MutProxy<'msg> for PayloadPlayerZoneEnterBeginMut<'msg> {}

impl<'msg> ::protobuf::AsView for PayloadPlayerZoneEnterBeginMut<'msg> {
  type Proxied = PayloadPlayerZoneEnterBegin;
  fn as_view(&self) -> ::protobuf::View<'_, PayloadPlayerZoneEnterBegin> {
    PayloadPlayerZoneEnterBeginView {
      inner: ::protobuf::__internal::runtime::MessageViewInner::view_of_mut(self.inner.clone()),
      _phantom: ::std::marker::PhantomData
    }
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for PayloadPlayerZoneEnterBeginMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, PayloadPlayerZoneEnterBegin>
  where
      'msg: 'shorter {
    PayloadPlayerZoneEnterBeginView {
      inner: ::protobuf::__internal::runtime::MessageViewInner::view_of_mut(self.inner.clone()),
      _phantom: ::std::marker::PhantomData
    }
  }
}

impl<'msg> ::protobuf::AsMut for PayloadPlayerZoneEnterBeginMut<'msg> {
  type MutProxied = PayloadPlayerZoneEnterBegin;
  fn as_mut(&mut self) -> PayloadPlayerZoneEnterBeginMut<'msg> {
    PayloadPlayerZoneEnterBeginMut { inner: self.inner }
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for PayloadPlayerZoneEnterBeginMut<'msg> {
  fn into_mut<'shorter>(self) -> PayloadPlayerZoneEnterBeginMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl PayloadPlayerZoneEnterBegin {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }

  fn raw_msg(&self) -> ::protobuf::__internal::runtime::RawMessage {
    self.inner.raw()
  }

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, PayloadPlayerZoneEnterBegin> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
  }

  fn arena(&mut self) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }

  pub fn parse(data: &[u8]) -> ::std::result::Result<Self, ::protobuf::ParseError> {
    let mut msg = Self::new();
    ::protobuf::ClearAndParse::clear_and_parse(&mut msg, data).map(|_| msg)
  }

  pub fn parse_dont_enforce_required(data: &[u8]) -> ::std::result::Result<Self, ::protobuf::ParseError> {
    let mut msg = Self::new();
    ::protobuf::ClearAndParse::clear_and_parse_dont_enforce_required(&mut msg, data).map(|_| msg)
  }

  pub fn as_view(&self) -> PayloadPlayerZoneEnterBeginView {
    PayloadPlayerZoneEnterBeginView::new(
        ::protobuf::__internal::Private,
        ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner))
  }

  pub fn as_mut(&mut self) -> PayloadPlayerZoneEnterBeginMut {
    let inner = ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner);
    PayloadPlayerZoneEnterBeginMut::new(::protobuf::__internal::Private, inner)
  }

  // user_id: optional uint64
  pub fn user_id(&self) -> u64 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_u64_at_index(
        0, (0u64).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_user_id(&mut self, val: u64) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_u64_at_index(
        0, val.into()
      )
    }
  }

  // gateway_id: optional uint64
  pub fn gateway_id(&self) -> u64 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_u64_at_index(
        1, (0u64).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_gateway_id(&mut self, val: u64) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_u64_at_index(
        1, val.into()
      )
    }
  }

}  // impl PayloadPlayerZoneEnterBegin

impl ::std::ops::Drop for PayloadPlayerZoneEnterBegin {
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for PayloadPlayerZoneEnterBegin {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for PayloadPlayerZoneEnterBegin {
  type Proxied = Self;
  fn as_view(&self) -> PayloadPlayerZoneEnterBeginView {
    self.as_view()
  }
}

impl ::protobuf::AsMut for PayloadPlayerZoneEnterBegin {
  type MutProxied = Self;
  fn as_mut(&mut self) -> PayloadPlayerZoneEnterBeginMut {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for PayloadPlayerZoneEnterBegin {
  fn mini_table() -> *const ::protobuf::__internal::runtime::upb_MiniTable {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTablePtr> =
        ::std::sync::OnceLock::new();
    ONCE_LOCK.get_or_init(|| unsafe {
      super::Proto__PayloadPlayerZoneEnterBegin_msg_init.0 =
          ::protobuf::__internal::runtime::upb_MiniTable_Build(
              "$,P,P".as_ptr(),
              5,
              ::protobuf::__internal::runtime::THREAD_LOCAL_ARENA.with(|a| a.raw()),
              ::std::ptr::null_mut());
      let submessages = [
      ];
      let subenums = [
      ];
      assert!(::protobuf::__internal::runtime::upb_MiniTable_Link(
          super::Proto__PayloadPlayerZoneEnterBegin_msg_init.0,
          submessages.as_ptr() as *const *const ::protobuf::__internal::runtime::upb_MiniTable,
          submessages.len(), subenums.as_ptr(), subenums.len()));
      ::protobuf::__internal::runtime::MiniTablePtr(super::Proto__PayloadPlayerZoneEnterBegin_msg_init.0)
    }).0
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for PayloadPlayerZoneEnterBegin {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for PayloadPlayerZoneEnterBeginView<'_> {
  #[inline(always)]
  fn mini_table() -> *const ::protobuf::__internal::runtime::upb_MiniTable {
    <PayloadPlayerZoneEnterBegin as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for PayloadPlayerZoneEnterBeginMut<'_> {
  #[inline(always)]
  fn mini_table() -> *const ::protobuf::__internal::runtime::upb_MiniTable {
    <PayloadPlayerZoneEnterBegin as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for PayloadPlayerZoneEnterBegin {
  type Msg = PayloadPlayerZoneEnterBegin;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<PayloadPlayerZoneEnterBegin> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for PayloadPlayerZoneEnterBegin {
  type Msg = PayloadPlayerZoneEnterBegin;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<PayloadPlayerZoneEnterBegin> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for PayloadPlayerZoneEnterBeginMut<'_> {
  type Msg = PayloadPlayerZoneEnterBegin;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<PayloadPlayerZoneEnterBegin> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for PayloadPlayerZoneEnterBeginMut<'_> {
  type Msg = PayloadPlayerZoneEnterBegin;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<PayloadPlayerZoneEnterBegin> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for PayloadPlayerZoneEnterBeginView<'_> {
  type Msg = PayloadPlayerZoneEnterBegin;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<PayloadPlayerZoneEnterBegin> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for PayloadPlayerZoneEnterBeginMut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}


// upb kernel doesn't support any owned message or message mut interop.
impl ::protobuf::OwnedMessageInterop for PayloadPlayerZoneEnterBegin {}
impl<'a> ::protobuf::MessageMutInterop<'a> for PayloadPlayerZoneEnterBeginMut<'a> {}

impl<'a> ::protobuf::MessageViewInterop<'a> for PayloadPlayerZoneEnterBeginView<'a> {
  unsafe fn __unstable_wrap_raw_message(
    msg: &'a *const ::std::ffi::c_void) -> Self {
    let raw = ::protobuf::__internal::runtime::RawMessage::new(*msg as *mut _).unwrap();
    let inner = unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap_raw(raw) };
    Self::new(::protobuf::__internal::Private, inner)
  }
  unsafe fn __unstable_wrap_raw_message_unchecked_lifetime(
    msg: *const ::std::ffi::c_void) -> Self {
    let raw = ::protobuf::__internal::runtime::RawMessage::new(msg as *mut _).unwrap();
    let inner = unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap_raw(raw) };
    Self::new(::protobuf::__internal::Private, inner)
  }
  fn __unstable_as_raw_message(&self) -> *const ::std::ffi::c_void {
    self.inner.raw().as_ptr() as *const _
  }
}

// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut Proto__PayloadPlayerZoneEnterReady_msg_init: ::protobuf::__internal::runtime::MiniTablePtr =
    ::protobuf::__internal::runtime::MiniTablePtr(::std::ptr::null_mut());
#[allow(non_camel_case_types)]
pub struct PayloadPlayerZoneEnterReady {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<PayloadPlayerZoneEnterReady>
}

impl ::protobuf::Message for PayloadPlayerZoneEnterReady {}

impl ::std::default::Default for PayloadPlayerZoneEnterReady {
  fn default() -> Self {
    Self::new()
  }
}

impl ::protobuf::Parse for PayloadPlayerZoneEnterReady {
  fn parse(serialized: &[u8]) -> ::std::result::Result<Self, ::protobuf::ParseError> {
    Self::parse(serialized)
  }

  fn parse_dont_enforce_required(serialized: &[u8]) -> ::std::result::Result<Self, ::protobuf::ParseError> {
    Self::parse_dont_enforce_required(serialized)
  }
}

impl ::std::fmt::Debug for PayloadPlayerZoneEnterReady {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    let string = unsafe {
      ::protobuf::__internal::runtime::debug_string(
        self.raw_msg(),
        <Self as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table()
      )
    };
    write!(f, "{}", string)
  }
}

impl ::protobuf::Serialize for PayloadPlayerZoneEnterReady {
  fn serialize(&self) -> ::std::result::Result<Vec<u8>, ::protobuf::SerializeError> {
    ::protobuf::AsView::as_view(self).serialize()
  }
}

// SAFETY:
// - `PayloadPlayerZoneEnterReady` is `Sync` because it does not implement interior mutability.
//    Neither does `PayloadPlayerZoneEnterReadyMut`.
unsafe impl Sync for PayloadPlayerZoneEnterReady {}

// SAFETY:
// - `PayloadPlayerZoneEnterReady` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl Send for PayloadPlayerZoneEnterReady {}

impl ::protobuf::Proxied for PayloadPlayerZoneEnterReady {
  type View<'msg> = PayloadPlayerZoneEnterReadyView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for PayloadPlayerZoneEnterReady {}

impl ::protobuf::MutProxied for PayloadPlayerZoneEnterReady {
  type Mut<'msg> = PayloadPlayerZoneEnterReadyMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct PayloadPlayerZoneEnterReadyView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, PayloadPlayerZoneEnterReady>,
  _phantom: ::std::marker::PhantomData<&'msg ()>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for PayloadPlayerZoneEnterReadyView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for PayloadPlayerZoneEnterReadyView<'msg> {
  type Message = PayloadPlayerZoneEnterReady;
}

impl ::std::fmt::Debug for PayloadPlayerZoneEnterReadyView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    let string = unsafe {
      ::protobuf::__internal::runtime::debug_string(
        self.raw_msg(),
        <Self as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table()
      )
    };
    write!(f, "{}", string)
  }
}

impl ::protobuf::Serialize for PayloadPlayerZoneEnterReadyView<'_> {
  fn serialize(&self) -> ::std::result::Result<Vec<u8>, ::protobuf::SerializeError> {
    // SAFETY: `MINI_TABLE` is the one associated with `self.raw_msg()`.
    let encoded = unsafe {
      ::protobuf::__internal::runtime::wire::encode(self.raw_msg(),
          <Self as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table())
    };
    encoded.map_err(|_| ::protobuf::SerializeError)
  }
}

impl ::std::default::Default for PayloadPlayerZoneEnterReadyView<'_> {
  fn default() -> PayloadPlayerZoneEnterReadyView<'static> {
    let inner = unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap_raw(::protobuf::__internal::runtime::ScratchSpace::zeroed_block()) };
    PayloadPlayerZoneEnterReadyView::new(::protobuf::__internal::Private, inner)
  }
}

#[allow(dead_code)]
impl<'msg> PayloadPlayerZoneEnterReadyView<'msg> {
  #[doc(hidden)]
  pub fn new(_private: ::protobuf::__internal::Private, inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, PayloadPlayerZoneEnterReady>) -> Self {
    Self { inner, _phantom: ::std::marker::PhantomData }
  }

  fn raw_msg(&self) -> ::protobuf::__internal::runtime::RawMessage {
    self.inner.raw()
  }

  pub fn to_owned(&self) -> PayloadPlayerZoneEnterReady {
    ::protobuf::IntoProxied::into_proxied(*self, ::protobuf::__internal::Private)
  }

  // user_id: optional uint64
  pub fn user_id(self) -> u64 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_u64_at_index(
        0, (0u64).into()
      ).try_into().unwrap()
    }
  }

}

// SAFETY:
// - `PayloadPlayerZoneEnterReadyView` is `Sync` because it does not support mutation.
unsafe impl Sync for PayloadPlayerZoneEnterReadyView<'_> {}

// SAFETY:
// - `PayloadPlayerZoneEnterReadyView` is `Send` because while its alive a `PayloadPlayerZoneEnterReadyMut` cannot.
// - `PayloadPlayerZoneEnterReadyView` does not use thread-local data.
unsafe impl Send for PayloadPlayerZoneEnterReadyView<'_> {}

impl<'msg> ::protobuf::Proxy<'msg> for PayloadPlayerZoneEnterReadyView<'msg> {}
impl<'msg> ::protobuf::ViewProxy<'msg> for PayloadPlayerZoneEnterReadyView<'msg> {}

impl<'msg> ::protobuf::AsView for PayloadPlayerZoneEnterReadyView<'msg> {
  type Proxied = PayloadPlayerZoneEnterReady;
  fn as_view(&self) -> ::protobuf::View<'msg, PayloadPlayerZoneEnterReady> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for PayloadPlayerZoneEnterReadyView<'msg> {
  fn into_view<'shorter>(self) -> PayloadPlayerZoneEnterReadyView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<PayloadPlayerZoneEnterReady> for PayloadPlayerZoneEnterReadyView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> PayloadPlayerZoneEnterReady {
    let mut dst = PayloadPlayerZoneEnterReady::new();
    let dst_raw = ::protobuf::__internal::runtime::UpbGetMessagePtrMut::get_raw_message_mut(&mut dst, ::protobuf::__internal::Private);
    let dst_arena = ::protobuf::__internal::runtime::UpbGetArena::get_arena(&mut dst, ::protobuf::__internal::Private);
    let src_raw = ::protobuf::__internal::runtime::UpbGetMessagePtr::get_raw_message(&self, ::protobuf::__internal::Private);

    unsafe { ::protobuf::__internal::runtime::upb_Message_DeepCopy(
      dst_raw,
      src_raw,
      <Self as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
      dst_arena.raw(),
    ) };
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<PayloadPlayerZoneEnterReady> for PayloadPlayerZoneEnterReadyMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> PayloadPlayerZoneEnterReady {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::runtime::UpbTypeConversions for PayloadPlayerZoneEnterReady {
    fn upb_type() -> ::protobuf::__internal::runtime::CType {
        ::protobuf::__internal::runtime::CType::Message
    }

    fn to_message_value(
        val: ::protobuf::View<'_, Self>) -> ::protobuf::__internal::runtime::upb_MessageValue {
        ::protobuf::__internal::runtime::upb_MessageValue { msg_val: Some(val.raw_msg()) }
    }

    unsafe fn into_message_value_fuse_if_required(
      raw_parent_arena: ::protobuf::__internal::runtime::RawArena,
      mut val: Self) -> ::protobuf::__internal::runtime::upb_MessageValue {
      // SAFETY: The arena memory is not freed due to `ManuallyDrop`.
      let parent_arena = ::std::mem::ManuallyDrop::new(
          unsafe { ::protobuf::__internal::runtime::Arena::from_raw(raw_parent_arena) });

      parent_arena.fuse(val.as_message_mut_inner(::protobuf::__internal::Private).arena());
      ::protobuf::__internal::runtime::upb_MessageValue { msg_val: Some(val.raw_msg()) }
    }

    unsafe fn from_message_value<'msg>(msg: ::protobuf::__internal::runtime::upb_MessageValue)
        -> ::protobuf::View<'msg, Self> {
        let raw = unsafe { msg.msg_val }.expect("expected present message value in map");
        let inner = unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap_raw(raw) };
        PayloadPlayerZoneEnterReadyView::new(::protobuf::__internal::Private, inner)
    }

    unsafe fn from_message_mut<'msg>(msg: ::protobuf::__internal::runtime::RawMessage, arena: &'msg ::protobuf::__internal::runtime::Arena)
        -> PayloadPlayerZoneEnterReadyMut<'msg> {
        let inner = unsafe { ::protobuf::__internal::runtime::MessageMutInner::<'msg, PayloadPlayerZoneEnterReady>::wrap_raw(msg, arena) };
        PayloadPlayerZoneEnterReadyMut::new(::protobuf::__internal::Private, inner)
    }
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct PayloadPlayerZoneEnterReadyMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, PayloadPlayerZoneEnterReady>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for PayloadPlayerZoneEnterReadyMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for PayloadPlayerZoneEnterReadyMut<'msg> {
  type Message = PayloadPlayerZoneEnterReady;
}

impl ::std::fmt::Debug for PayloadPlayerZoneEnterReadyMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    let string = unsafe {
      ::protobuf::__internal::runtime::debug_string(
        self.raw_msg(),
        <Self as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table()
      )
    };
    write!(f, "{}", string)
  }
}

impl ::protobuf::Serialize for PayloadPlayerZoneEnterReadyMut<'_> {
  fn serialize(&self) -> ::std::result::Result<Vec<u8>, ::protobuf::SerializeError> {
    ::protobuf::AsView::as_view(self).serialize()
  }
}

#[allow(dead_code)]
impl<'msg> PayloadPlayerZoneEnterReadyMut<'msg> {
  #[doc(hidden)]
  pub fn from_parent<ParentT: ::protobuf::Message>(
             _private: ::protobuf::__internal::Private,
             parent: ::protobuf::__internal::runtime::MessageMutInner<'msg, ParentT>,
             msg: ::protobuf::__internal::runtime::RawMessage)
    -> Self {
    Self {
      inner: ::protobuf::__internal::runtime::MessageMutInner::from_parent(parent, msg)
    }
  }

  #[doc(hidden)]
  pub fn new(_private: ::protobuf::__internal::Private, inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, PayloadPlayerZoneEnterReady>) -> Self {
    Self { inner }
  }

  fn raw_msg(&self) -> ::protobuf::__internal::runtime::RawMessage {
    self.inner.raw()
  }

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, PayloadPlayerZoneEnterReady> {
    self.inner
  }

  pub fn to_owned(&self) -> PayloadPlayerZoneEnterReady {
    ::protobuf::AsView::as_view(self).to_owned()
  }

  fn arena(&mut self) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }

  // user_id: optional uint64
  pub fn user_id(&self) -> u64 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_u64_at_index(
        0, (0u64).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_user_id(&mut self, val: u64) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_u64_at_index(
        0, val.into()
      )
    }
  }

}

// SAFETY:
// - `PayloadPlayerZoneEnterReadyMut` does not perform any shared mutation.
// - `PayloadPlayerZoneEnterReadyMut` is not `Send`, and so even in the presence of mutator
//   splitting, synchronous access of an arena is impossible.
unsafe impl Sync for PayloadPlayerZoneEnterReadyMut<'_> {}

impl<'msg> ::protobuf::Proxy<'msg> for PayloadPlayerZoneEnterReadyMut<'msg> {}
impl<'msg> ::protobuf::MutProxy<'msg> for PayloadPlayerZoneEnterReadyMut<'msg> {}

impl<'msg> ::protobuf::AsView for PayloadPlayerZoneEnterReadyMut<'msg> {
  type Proxied = PayloadPlayerZoneEnterReady;
  fn as_view(&self) -> ::protobuf::View<'_, PayloadPlayerZoneEnterReady> {
    PayloadPlayerZoneEnterReadyView {
      inner: ::protobuf::__internal::runtime::MessageViewInner::view_of_mut(self.inner.clone()),
      _phantom: ::std::marker::PhantomData
    }
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for PayloadPlayerZoneEnterReadyMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, PayloadPlayerZoneEnterReady>
  where
      'msg: 'shorter {
    PayloadPlayerZoneEnterReadyView {
      inner: ::protobuf::__internal::runtime::MessageViewInner::view_of_mut(self.inner.clone()),
      _phantom: ::std::marker::PhantomData
    }
  }
}

impl<'msg> ::protobuf::AsMut for PayloadPlayerZoneEnterReadyMut<'msg> {
  type MutProxied = PayloadPlayerZoneEnterReady;
  fn as_mut(&mut self) -> PayloadPlayerZoneEnterReadyMut<'msg> {
    PayloadPlayerZoneEnterReadyMut { inner: self.inner }
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for PayloadPlayerZoneEnterReadyMut<'msg> {
  fn into_mut<'shorter>(self) -> PayloadPlayerZoneEnterReadyMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl PayloadPlayerZoneEnterReady {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }

  fn raw_msg(&self) -> ::protobuf::__internal::runtime::RawMessage {
    self.inner.raw()
  }

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, PayloadPlayerZoneEnterReady> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
  }

  fn arena(&mut self) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }

  pub fn parse(data: &[u8]) -> ::std::result::Result<Self, ::protobuf::ParseError> {
    let mut msg = Self::new();
    ::protobuf::ClearAndParse::clear_and_parse(&mut msg, data).map(|_| msg)
  }

  pub fn parse_dont_enforce_required(data: &[u8]) -> ::std::result::Result<Self, ::protobuf::ParseError> {
    let mut msg = Self::new();
    ::protobuf::ClearAndParse::clear_and_parse_dont_enforce_required(&mut msg, data).map(|_| msg)
  }

  pub fn as_view(&self) -> PayloadPlayerZoneEnterReadyView {
    PayloadPlayerZoneEnterReadyView::new(
        ::protobuf::__internal::Private,
        ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner))
  }

  pub fn as_mut(&mut self) -> PayloadPlayerZoneEnterReadyMut {
    let inner = ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner);
    PayloadPlayerZoneEnterReadyMut::new(::protobuf::__internal::Private, inner)
  }

  // user_id: optional uint64
  pub fn user_id(&self) -> u64 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_u64_at_index(
        0, (0u64).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_user_id(&mut self, val: u64) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_u64_at_index(
        0, val.into()
      )
    }
  }

}  // impl PayloadPlayerZoneEnterReady

impl ::std::ops::Drop for PayloadPlayerZoneEnterReady {
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for PayloadPlayerZoneEnterReady {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for PayloadPlayerZoneEnterReady {
  type Proxied = Self;
  fn as_view(&self) -> PayloadPlayerZoneEnterReadyView {
    self.as_view()
  }
}

impl ::protobuf::AsMut for PayloadPlayerZoneEnterReady {
  type MutProxied = Self;
  fn as_mut(&mut self) -> PayloadPlayerZoneEnterReadyMut {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for PayloadPlayerZoneEnterReady {
  fn mini_table() -> *const ::protobuf::__internal::runtime::upb_MiniTable {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTablePtr> =
        ::std::sync::OnceLock::new();
    ONCE_LOCK.get_or_init(|| unsafe {
      super::Proto__PayloadPlayerZoneEnterReady_msg_init.0 =
          ::protobuf::__internal::runtime::upb_MiniTable_Build(
              "$,P".as_ptr(),
              3,
              ::protobuf::__internal::runtime::THREAD_LOCAL_ARENA.with(|a| a.raw()),
              ::std::ptr::null_mut());
      let submessages = [
      ];
      let subenums = [
      ];
      assert!(::protobuf::__internal::runtime::upb_MiniTable_Link(
          super::Proto__PayloadPlayerZoneEnterReady_msg_init.0,
          submessages.as_ptr() as *const *const ::protobuf::__internal::runtime::upb_MiniTable,
          submessages.len(), subenums.as_ptr(), subenums.len()));
      ::protobuf::__internal::runtime::MiniTablePtr(super::Proto__PayloadPlayerZoneEnterReady_msg_init.0)
    }).0
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for PayloadPlayerZoneEnterReady {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for PayloadPlayerZoneEnterReadyView<'_> {
  #[inline(always)]
  fn mini_table() -> *const ::protobuf::__internal::runtime::upb_MiniTable {
    <PayloadPlayerZoneEnterReady as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for PayloadPlayerZoneEnterReadyMut<'_> {
  #[inline(always)]
  fn mini_table() -> *const ::protobuf::__internal::runtime::upb_MiniTable {
    <PayloadPlayerZoneEnterReady as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for PayloadPlayerZoneEnterReady {
  type Msg = PayloadPlayerZoneEnterReady;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<PayloadPlayerZoneEnterReady> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for PayloadPlayerZoneEnterReady {
  type Msg = PayloadPlayerZoneEnterReady;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<PayloadPlayerZoneEnterReady> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for PayloadPlayerZoneEnterReadyMut<'_> {
  type Msg = PayloadPlayerZoneEnterReady;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<PayloadPlayerZoneEnterReady> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for PayloadPlayerZoneEnterReadyMut<'_> {
  type Msg = PayloadPlayerZoneEnterReady;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<PayloadPlayerZoneEnterReady> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for PayloadPlayerZoneEnterReadyView<'_> {
  type Msg = PayloadPlayerZoneEnterReady;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<PayloadPlayerZoneEnterReady> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for PayloadPlayerZoneEnterReadyMut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}


// upb kernel doesn't support any owned message or message mut interop.
impl ::protobuf::OwnedMessageInterop for PayloadPlayerZoneEnterReady {}
impl<'a> ::protobuf::MessageMutInterop<'a> for PayloadPlayerZoneEnterReadyMut<'a> {}

impl<'a> ::protobuf::MessageViewInterop<'a> for PayloadPlayerZoneEnterReadyView<'a> {
  unsafe fn __unstable_wrap_raw_message(
    msg: &'a *const ::std::ffi::c_void) -> Self {
    let raw = ::protobuf::__internal::runtime::RawMessage::new(*msg as *mut _).unwrap();
    let inner = unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap_raw(raw) };
    Self::new(::protobuf::__internal::Private, inner)
  }
  unsafe fn __unstable_wrap_raw_message_unchecked_lifetime(
    msg: *const ::std::ffi::c_void) -> Self {
    let raw = ::protobuf::__internal::runtime::RawMessage::new(msg as *mut _).unwrap();
    let inner = unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap_raw(raw) };
    Self::new(::protobuf::__internal::Private, inner)
  }
  fn __unstable_as_raw_message(&self) -> *const ::std::ffi::c_void {
    self.inner.raw().as_ptr() as *const _
  }
}

// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut Proto__PayloadPlayerZoneEnterComplete_msg_init: ::protobuf::__internal::runtime::MiniTablePtr =
    ::protobuf::__internal::runtime::MiniTablePtr(::std::ptr::null_mut());
#[allow(non_camel_case_types)]
pub struct PayloadPlayerZoneEnterComplete {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<PayloadPlayerZoneEnterComplete>
}

impl ::protobuf::Message for PayloadPlayerZoneEnterComplete {}

impl ::std::default::Default for PayloadPlayerZoneEnterComplete {
  fn default() -> Self {
    Self::new()
  }
}

impl ::protobuf::Parse for PayloadPlayerZoneEnterComplete {
  fn parse(serialized: &[u8]) -> ::std::result::Result<Self, ::protobuf::ParseError> {
    Self::parse(serialized)
  }

  fn parse_dont_enforce_required(serialized: &[u8]) -> ::std::result::Result<Self, ::protobuf::ParseError> {
    Self::parse_dont_enforce_required(serialized)
  }
}

impl ::std::fmt::Debug for PayloadPlayerZoneEnterComplete {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    let string = unsafe {
      ::protobuf::__internal::runtime::debug_string(
        self.raw_msg(),
        <Self as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table()
      )
    };
    write!(f, "{}", string)
  }
}

impl ::protobuf::Serialize for PayloadPlayerZoneEnterComplete {
  fn serialize(&self) -> ::std::result::Result<Vec<u8>, ::protobuf::SerializeError> {
    ::protobuf::AsView::as_view(self).serialize()
  }
}

// SAFETY:
// - `PayloadPlayerZoneEnterComplete` is `Sync` because it does not implement interior mutability.
//    Neither does `PayloadPlayerZoneEnterCompleteMut`.
unsafe impl Sync for PayloadPlayerZoneEnterComplete {}

// SAFETY:
// - `PayloadPlayerZoneEnterComplete` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl Send for PayloadPlayerZoneEnterComplete {}

impl ::protobuf::Proxied for PayloadPlayerZoneEnterComplete {
  type View<'msg> = PayloadPlayerZoneEnterCompleteView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for PayloadPlayerZoneEnterComplete {}

impl ::protobuf::MutProxied for PayloadPlayerZoneEnterComplete {
  type Mut<'msg> = PayloadPlayerZoneEnterCompleteMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct PayloadPlayerZoneEnterCompleteView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, PayloadPlayerZoneEnterComplete>,
  _phantom: ::std::marker::PhantomData<&'msg ()>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for PayloadPlayerZoneEnterCompleteView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for PayloadPlayerZoneEnterCompleteView<'msg> {
  type Message = PayloadPlayerZoneEnterComplete;
}

impl ::std::fmt::Debug for PayloadPlayerZoneEnterCompleteView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    let string = unsafe {
      ::protobuf::__internal::runtime::debug_string(
        self.raw_msg(),
        <Self as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table()
      )
    };
    write!(f, "{}", string)
  }
}

impl ::protobuf::Serialize for PayloadPlayerZoneEnterCompleteView<'_> {
  fn serialize(&self) -> ::std::result::Result<Vec<u8>, ::protobuf::SerializeError> {
    // SAFETY: `MINI_TABLE` is the one associated with `self.raw_msg()`.
    let encoded = unsafe {
      ::protobuf::__internal::runtime::wire::encode(self.raw_msg(),
          <Self as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table())
    };
    encoded.map_err(|_| ::protobuf::SerializeError)
  }
}

impl ::std::default::Default for PayloadPlayerZoneEnterCompleteView<'_> {
  fn default() -> PayloadPlayerZoneEnterCompleteView<'static> {
    let inner = unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap_raw(::protobuf::__internal::runtime::ScratchSpace::zeroed_block()) };
    PayloadPlayerZoneEnterCompleteView::new(::protobuf::__internal::Private, inner)
  }
}

#[allow(dead_code)]
impl<'msg> PayloadPlayerZoneEnterCompleteView<'msg> {
  #[doc(hidden)]
  pub fn new(_private: ::protobuf::__internal::Private, inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, PayloadPlayerZoneEnterComplete>) -> Self {
    Self { inner, _phantom: ::std::marker::PhantomData }
  }

  fn raw_msg(&self) -> ::protobuf::__internal::runtime::RawMessage {
    self.inner.raw()
  }

  pub fn to_owned(&self) -> PayloadPlayerZoneEnterComplete {
    ::protobuf::IntoProxied::into_proxied(*self, ::protobuf::__internal::Private)
  }

  // user_id: optional uint64
  pub fn user_id(self) -> u64 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_u64_at_index(
        0, (0u64).into()
      ).try_into().unwrap()
    }
  }

}

// SAFETY:
// - `PayloadPlayerZoneEnterCompleteView` is `Sync` because it does not support mutation.
unsafe impl Sync for PayloadPlayerZoneEnterCompleteView<'_> {}

// SAFETY:
// - `PayloadPlayerZoneEnterCompleteView` is `Send` because while its alive a `PayloadPlayerZoneEnterCompleteMut` cannot.
// - `PayloadPlayerZoneEnterCompleteView` does not use thread-local data.
unsafe impl Send for PayloadPlayerZoneEnterCompleteView<'_> {}

impl<'msg> ::protobuf::Proxy<'msg> for PayloadPlayerZoneEnterCompleteView<'msg> {}
impl<'msg> ::protobuf::ViewProxy<'msg> for PayloadPlayerZoneEnterCompleteView<'msg> {}

impl<'msg> ::protobuf::AsView for PayloadPlayerZoneEnterCompleteView<'msg> {
  type Proxied = PayloadPlayerZoneEnterComplete;
  fn as_view(&self) -> ::protobuf::View<'msg, PayloadPlayerZoneEnterComplete> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for PayloadPlayerZoneEnterCompleteView<'msg> {
  fn into_view<'shorter>(self) -> PayloadPlayerZoneEnterCompleteView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<PayloadPlayerZoneEnterComplete> for PayloadPlayerZoneEnterCompleteView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> PayloadPlayerZoneEnterComplete {
    let mut dst = PayloadPlayerZoneEnterComplete::new();
    let dst_raw = ::protobuf::__internal::runtime::UpbGetMessagePtrMut::get_raw_message_mut(&mut dst, ::protobuf::__internal::Private);
    let dst_arena = ::protobuf::__internal::runtime::UpbGetArena::get_arena(&mut dst, ::protobuf::__internal::Private);
    let src_raw = ::protobuf::__internal::runtime::UpbGetMessagePtr::get_raw_message(&self, ::protobuf::__internal::Private);

    unsafe { ::protobuf::__internal::runtime::upb_Message_DeepCopy(
      dst_raw,
      src_raw,
      <Self as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
      dst_arena.raw(),
    ) };
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<PayloadPlayerZoneEnterComplete> for PayloadPlayerZoneEnterCompleteMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> PayloadPlayerZoneEnterComplete {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::runtime::UpbTypeConversions for PayloadPlayerZoneEnterComplete {
    fn upb_type() -> ::protobuf::__internal::runtime::CType {
        ::protobuf::__internal::runtime::CType::Message
    }

    fn to_message_value(
        val: ::protobuf::View<'_, Self>) -> ::protobuf::__internal::runtime::upb_MessageValue {
        ::protobuf::__internal::runtime::upb_MessageValue { msg_val: Some(val.raw_msg()) }
    }

    unsafe fn into_message_value_fuse_if_required(
      raw_parent_arena: ::protobuf::__internal::runtime::RawArena,
      mut val: Self) -> ::protobuf::__internal::runtime::upb_MessageValue {
      // SAFETY: The arena memory is not freed due to `ManuallyDrop`.
      let parent_arena = ::std::mem::ManuallyDrop::new(
          unsafe { ::protobuf::__internal::runtime::Arena::from_raw(raw_parent_arena) });

      parent_arena.fuse(val.as_message_mut_inner(::protobuf::__internal::Private).arena());
      ::protobuf::__internal::runtime::upb_MessageValue { msg_val: Some(val.raw_msg()) }
    }

    unsafe fn from_message_value<'msg>(msg: ::protobuf::__internal::runtime::upb_MessageValue)
        -> ::protobuf::View<'msg, Self> {
        let raw = unsafe { msg.msg_val }.expect("expected present message value in map");
        let inner = unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap_raw(raw) };
        PayloadPlayerZoneEnterCompleteView::new(::protobuf::__internal::Private, inner)
    }

    unsafe fn from_message_mut<'msg>(msg: ::protobuf::__internal::runtime::RawMessage, arena: &'msg ::protobuf::__internal::runtime::Arena)
        -> PayloadPlayerZoneEnterCompleteMut<'msg> {
        let inner = unsafe { ::protobuf::__internal::runtime::MessageMutInner::<'msg, PayloadPlayerZoneEnterComplete>::wrap_raw(msg, arena) };
        PayloadPlayerZoneEnterCompleteMut::new(::protobuf::__internal::Private, inner)
    }
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct PayloadPlayerZoneEnterCompleteMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, PayloadPlayerZoneEnterComplete>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for PayloadPlayerZoneEnterCompleteMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for PayloadPlayerZoneEnterCompleteMut<'msg> {
  type Message = PayloadPlayerZoneEnterComplete;
}

impl ::std::fmt::Debug for PayloadPlayerZoneEnterCompleteMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    let string = unsafe {
      ::protobuf::__internal::runtime::debug_string(
        self.raw_msg(),
        <Self as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table()
      )
    };
    write!(f, "{}", string)
  }
}

impl ::protobuf::Serialize for PayloadPlayerZoneEnterCompleteMut<'_> {
  fn serialize(&self) -> ::std::result::Result<Vec<u8>, ::protobuf::SerializeError> {
    ::protobuf::AsView::as_view(self).serialize()
  }
}

#[allow(dead_code)]
impl<'msg> PayloadPlayerZoneEnterCompleteMut<'msg> {
  #[doc(hidden)]
  pub fn from_parent<ParentT: ::protobuf::Message>(
             _private: ::protobuf::__internal::Private,
             parent: ::protobuf::__internal::runtime::MessageMutInner<'msg, ParentT>,
             msg: ::protobuf::__internal::runtime::RawMessage)
    -> Self {
    Self {
      inner: ::protobuf::__internal::runtime::MessageMutInner::from_parent(parent, msg)
    }
  }

  #[doc(hidden)]
  pub fn new(_private: ::protobuf::__internal::Private, inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, PayloadPlayerZoneEnterComplete>) -> Self {
    Self { inner }
  }

  fn raw_msg(&self) -> ::protobuf::__internal::runtime::RawMessage {
    self.inner.raw()
  }

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, PayloadPlayerZoneEnterComplete> {
    self.inner
  }

  pub fn to_owned(&self) -> PayloadPlayerZoneEnterComplete {
    ::protobuf::AsView::as_view(self).to_owned()
  }

  fn arena(&mut self) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }

  // user_id: optional uint64
  pub fn user_id(&self) -> u64 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_u64_at_index(
        0, (0u64).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_user_id(&mut self, val: u64) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_u64_at_index(
        0, val.into()
      )
    }
  }

}

// SAFETY:
// - `PayloadPlayerZoneEnterCompleteMut` does not perform any shared mutation.
// - `PayloadPlayerZoneEnterCompleteMut` is not `Send`, and so even in the presence of mutator
//   splitting, synchronous access of an arena is impossible.
unsafe impl Sync for PayloadPlayerZoneEnterCompleteMut<'_> {}

impl<'msg> ::protobuf::Proxy<'msg> for PayloadPlayerZoneEnterCompleteMut<'msg> {}
impl<'msg> ::protobuf::MutProxy<'msg> for PayloadPlayerZoneEnterCompleteMut<'msg> {}

impl<'msg> ::protobuf::AsView for PayloadPlayerZoneEnterCompleteMut<'msg> {
  type Proxied = PayloadPlayerZoneEnterComplete;
  fn as_view(&self) -> ::protobuf::View<'_, PayloadPlayerZoneEnterComplete> {
    PayloadPlayerZoneEnterCompleteView {
      inner: ::protobuf::__internal::runtime::MessageViewInner::view_of_mut(self.inner.clone()),
      _phantom: ::std::marker::PhantomData
    }
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for PayloadPlayerZoneEnterCompleteMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, PayloadPlayerZoneEnterComplete>
  where
      'msg: 'shorter {
    PayloadPlayerZoneEnterCompleteView {
      inner: ::protobuf::__internal::runtime::MessageViewInner::view_of_mut(self.inner.clone()),
      _phantom: ::std::marker::PhantomData
    }
  }
}

impl<'msg> ::protobuf::AsMut for PayloadPlayerZoneEnterCompleteMut<'msg> {
  type MutProxied = PayloadPlayerZoneEnterComplete;
  fn as_mut(&mut self) -> PayloadPlayerZoneEnterCompleteMut<'msg> {
    PayloadPlayerZoneEnterCompleteMut { inner: self.inner }
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for PayloadPlayerZoneEnterCompleteMut<'msg> {
  fn into_mut<'shorter>(self) -> PayloadPlayerZoneEnterCompleteMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl PayloadPlayerZoneEnterComplete {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }

  fn raw_msg(&self) -> ::protobuf::__internal::runtime::RawMessage {
    self.inner.raw()
  }

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, PayloadPlayerZoneEnterComplete> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
  }

  fn arena(&mut self) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }

  pub fn parse(data: &[u8]) -> ::std::result::Result<Self, ::protobuf::ParseError> {
    let mut msg = Self::new();
    ::protobuf::ClearAndParse::clear_and_parse(&mut msg, data).map(|_| msg)
  }

  pub fn parse_dont_enforce_required(data: &[u8]) -> ::std::result::Result<Self, ::protobuf::ParseError> {
    let mut msg = Self::new();
    ::protobuf::ClearAndParse::clear_and_parse_dont_enforce_required(&mut msg, data).map(|_| msg)
  }

  pub fn as_view(&self) -> PayloadPlayerZoneEnterCompleteView {
    PayloadPlayerZoneEnterCompleteView::new(
        ::protobuf::__internal::Private,
        ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner))
  }

  pub fn as_mut(&mut self) -> PayloadPlayerZoneEnterCompleteMut {
    let inner = ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner);
    PayloadPlayerZoneEnterCompleteMut::new(::protobuf::__internal::Private, inner)
  }

  // user_id: optional uint64
  pub fn user_id(&self) -> u64 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_u64_at_index(
        0, (0u64).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_user_id(&mut self, val: u64) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_u64_at_index(
        0, val.into()
      )
    }
  }

}  // impl PayloadPlayerZoneEnterComplete

impl ::std::ops::Drop for PayloadPlayerZoneEnterComplete {
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for PayloadPlayerZoneEnterComplete {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for PayloadPlayerZoneEnterComplete {
  type Proxied = Self;
  fn as_view(&self) -> PayloadPlayerZoneEnterCompleteView {
    self.as_view()
  }
}

impl ::protobuf::AsMut for PayloadPlayerZoneEnterComplete {
  type MutProxied = Self;
  fn as_mut(&mut self) -> PayloadPlayerZoneEnterCompleteMut {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for PayloadPlayerZoneEnterComplete {
  fn mini_table() -> *const ::protobuf::__internal::runtime::upb_MiniTable {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTablePtr> =
        ::std::sync::OnceLock::new();
    ONCE_LOCK.get_or_init(|| unsafe {
      super::Proto__PayloadPlayerZoneEnterComplete_msg_init.0 =
          ::protobuf::__internal::runtime::upb_MiniTable_Build(
              "$,P".as_ptr(),
              3,
              ::protobuf::__internal::runtime::THREAD_LOCAL_ARENA.with(|a| a.raw()),
              ::std::ptr::null_mut());
      let submessages = [
      ];
      let subenums = [
      ];
      assert!(::protobuf::__internal::runtime::upb_MiniTable_Link(
          super::Proto__PayloadPlayerZoneEnterComplete_msg_init.0,
          submessages.as_ptr() as *const *const ::protobuf::__internal::runtime::upb_MiniTable,
          submessages.len(), subenums.as_ptr(), subenums.len()));
      ::protobuf::__internal::runtime::MiniTablePtr(super::Proto__PayloadPlayerZoneEnterComplete_msg_init.0)
    }).0
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for PayloadPlayerZoneEnterComplete {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for PayloadPlayerZoneEnterCompleteView<'_> {
  #[inline(always)]
  fn mini_table() -> *const ::protobuf::__internal::runtime::upb_MiniTable {
    <PayloadPlayerZoneEnterComplete as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for PayloadPlayerZoneEnterCompleteMut<'_> {
  #[inline(always)]
  fn mini_table() -> *const ::protobuf::__internal::runtime::upb_MiniTable {
    <PayloadPlayerZoneEnterComplete as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for PayloadPlayerZoneEnterComplete {
  type Msg = PayloadPlayerZoneEnterComplete;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<PayloadPlayerZoneEnterComplete> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for PayloadPlayerZoneEnterComplete {
  type Msg = PayloadPlayerZoneEnterComplete;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<PayloadPlayerZoneEnterComplete> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for PayloadPlayerZoneEnterCompleteMut<'_> {
  type Msg = PayloadPlayerZoneEnterComplete;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<PayloadPlayerZoneEnterComplete> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for PayloadPlayerZoneEnterCompleteMut<'_> {
  type Msg = PayloadPlayerZoneEnterComplete;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<PayloadPlayerZoneEnterComplete> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for PayloadPlayerZoneEnterCompleteView<'_> {
  type Msg = PayloadPlayerZoneEnterComplete;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<PayloadPlayerZoneEnterComplete> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for PayloadPlayerZoneEnterCompleteMut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}


// upb kernel doesn't support any owned message or message mut interop.
impl ::protobuf::OwnedMessageInterop for PayloadPlayerZoneEnterComplete {}
impl<'a> ::protobuf::MessageMutInterop<'a> for PayloadPlayerZoneEnterCompleteMut<'a> {}

impl<'a> ::protobuf::MessageViewInterop<'a> for PayloadPlayerZoneEnterCompleteView<'a> {
  unsafe fn __unstable_wrap_raw_message(
    msg: &'a *const ::std::ffi::c_void) -> Self {
    let raw = ::protobuf::__internal::runtime::RawMessage::new(*msg as *mut _).unwrap();
    let inner = unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap_raw(raw) };
    Self::new(::protobuf::__internal::Private, inner)
  }
  unsafe fn __unstable_wrap_raw_message_unchecked_lifetime(
    msg: *const ::std::ffi::c_void) -> Self {
    let raw = ::protobuf::__internal::runtime::RawMessage::new(msg as *mut _).unwrap();
    let inner = unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap_raw(raw) };
    Self::new(::protobuf::__internal::Private, inner)
  }
  fn __unstable_as_raw_message(&self) -> *const ::std::ffi::c_void {
    self.inner.raw().as_ptr() as *const _
  }
}

// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut Proto__PayloadPlayerZoneEnterReadyResponse_msg_init: ::protobuf::__internal::runtime::MiniTablePtr =
    ::protobuf::__internal::runtime::MiniTablePtr(::std::ptr::null_mut());
#[allow(non_camel_case_types)]
pub struct PayloadPlayerZoneEnterReadyResponse {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<PayloadPlayerZoneEnterReadyResponse>
}

impl ::protobuf::Message for PayloadPlayerZoneEnterReadyResponse {}

impl ::std::default::Default for PayloadPlayerZoneEnterReadyResponse {
  fn default() -> Self {
    Self::new()
  }
}

impl ::protobuf::Parse for PayloadPlayerZoneEnterReadyResponse {
  fn parse(serialized: &[u8]) -> ::std::result::Result<Self, ::protobuf::ParseError> {
    Self::parse(serialized)
  }

  fn parse_dont_enforce_required(serialized: &[u8]) -> ::std::result::Result<Self, ::protobuf::ParseError> {
    Self::parse_dont_enforce_required(serialized)
  }
}

impl ::std::fmt::Debug for PayloadPlayerZoneEnterReadyResponse {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    let string = unsafe {
      ::protobuf::__internal::runtime::debug_string(
        self.raw_msg(),
        <Self as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table()
      )
    };
    write!(f, "{}", string)
  }
}

impl ::protobuf::Serialize for PayloadPlayerZoneEnterReadyResponse {
  fn serialize(&self) -> ::std::result::Result<Vec<u8>, ::protobuf::SerializeError> {
    ::protobuf::AsView::as_view(self).serialize()
  }
}

// SAFETY:
// - `PayloadPlayerZoneEnterReadyResponse` is `Sync` because it does not implement interior mutability.
//    Neither does `PayloadPlayerZoneEnterReadyResponseMut`.
unsafe impl Sync for PayloadPlayerZoneEnterReadyResponse {}

// SAFETY:
// - `PayloadPlayerZoneEnterReadyResponse` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl Send for PayloadPlayerZoneEnterReadyResponse {}

impl ::protobuf::Proxied for PayloadPlayerZoneEnterReadyResponse {
  type View<'msg> = PayloadPlayerZoneEnterReadyResponseView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for PayloadPlayerZoneEnterReadyResponse {}

impl ::protobuf::MutProxied for PayloadPlayerZoneEnterReadyResponse {
  type Mut<'msg> = PayloadPlayerZoneEnterReadyResponseMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct PayloadPlayerZoneEnterReadyResponseView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, PayloadPlayerZoneEnterReadyResponse>,
  _phantom: ::std::marker::PhantomData<&'msg ()>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for PayloadPlayerZoneEnterReadyResponseView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for PayloadPlayerZoneEnterReadyResponseView<'msg> {
  type Message = PayloadPlayerZoneEnterReadyResponse;
}

impl ::std::fmt::Debug for PayloadPlayerZoneEnterReadyResponseView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    let string = unsafe {
      ::protobuf::__internal::runtime::debug_string(
        self.raw_msg(),
        <Self as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table()
      )
    };
    write!(f, "{}", string)
  }
}

impl ::protobuf::Serialize for PayloadPlayerZoneEnterReadyResponseView<'_> {
  fn serialize(&self) -> ::std::result::Result<Vec<u8>, ::protobuf::SerializeError> {
    // SAFETY: `MINI_TABLE` is the one associated with `self.raw_msg()`.
    let encoded = unsafe {
      ::protobuf::__internal::runtime::wire::encode(self.raw_msg(),
          <Self as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table())
    };
    encoded.map_err(|_| ::protobuf::SerializeError)
  }
}

impl ::std::default::Default for PayloadPlayerZoneEnterReadyResponseView<'_> {
  fn default() -> PayloadPlayerZoneEnterReadyResponseView<'static> {
    let inner = unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap_raw(::protobuf::__internal::runtime::ScratchSpace::zeroed_block()) };
    PayloadPlayerZoneEnterReadyResponseView::new(::protobuf::__internal::Private, inner)
  }
}

#[allow(dead_code)]
impl<'msg> PayloadPlayerZoneEnterReadyResponseView<'msg> {
  #[doc(hidden)]
  pub fn new(_private: ::protobuf::__internal::Private, inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, PayloadPlayerZoneEnterReadyResponse>) -> Self {
    Self { inner, _phantom: ::std::marker::PhantomData }
  }

  fn raw_msg(&self) -> ::protobuf::__internal::runtime::RawMessage {
    self.inner.raw()
  }

  pub fn to_owned(&self) -> PayloadPlayerZoneEnterReadyResponse {
    ::protobuf::IntoProxied::into_proxied(*self, ::protobuf::__internal::Private)
  }

  // player_data: optional message Proto.RoutePlayerData
  pub fn has_player_data(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn player_data_opt(self) -> ::protobuf::Optional<super::RoutePlayerDataView<'msg>> {
        ::protobuf::Optional::new(self.player_data(), self.has_player_data())
  }
  pub fn player_data(self) -> super::RoutePlayerDataView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(0)
    };
    let raw = submsg.map(|ptr| ptr.raw()).unwrap_or(::protobuf::__internal::runtime::ScratchSpace::zeroed_block());
    let inner = unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap_raw(raw) };
    super::RoutePlayerDataView::new(::protobuf::__internal::Private, inner)
  }

}

// SAFETY:
// - `PayloadPlayerZoneEnterReadyResponseView` is `Sync` because it does not support mutation.
unsafe impl Sync for PayloadPlayerZoneEnterReadyResponseView<'_> {}

// SAFETY:
// - `PayloadPlayerZoneEnterReadyResponseView` is `Send` because while its alive a `PayloadPlayerZoneEnterReadyResponseMut` cannot.
// - `PayloadPlayerZoneEnterReadyResponseView` does not use thread-local data.
unsafe impl Send for PayloadPlayerZoneEnterReadyResponseView<'_> {}

impl<'msg> ::protobuf::Proxy<'msg> for PayloadPlayerZoneEnterReadyResponseView<'msg> {}
impl<'msg> ::protobuf::ViewProxy<'msg> for PayloadPlayerZoneEnterReadyResponseView<'msg> {}

impl<'msg> ::protobuf::AsView for PayloadPlayerZoneEnterReadyResponseView<'msg> {
  type Proxied = PayloadPlayerZoneEnterReadyResponse;
  fn as_view(&self) -> ::protobuf::View<'msg, PayloadPlayerZoneEnterReadyResponse> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for PayloadPlayerZoneEnterReadyResponseView<'msg> {
  fn into_view<'shorter>(self) -> PayloadPlayerZoneEnterReadyResponseView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<PayloadPlayerZoneEnterReadyResponse> for PayloadPlayerZoneEnterReadyResponseView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> PayloadPlayerZoneEnterReadyResponse {
    let mut dst = PayloadPlayerZoneEnterReadyResponse::new();
    let dst_raw = ::protobuf::__internal::runtime::UpbGetMessagePtrMut::get_raw_message_mut(&mut dst, ::protobuf::__internal::Private);
    let dst_arena = ::protobuf::__internal::runtime::UpbGetArena::get_arena(&mut dst, ::protobuf::__internal::Private);
    let src_raw = ::protobuf::__internal::runtime::UpbGetMessagePtr::get_raw_message(&self, ::protobuf::__internal::Private);

    unsafe { ::protobuf::__internal::runtime::upb_Message_DeepCopy(
      dst_raw,
      src_raw,
      <Self as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
      dst_arena.raw(),
    ) };
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<PayloadPlayerZoneEnterReadyResponse> for PayloadPlayerZoneEnterReadyResponseMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> PayloadPlayerZoneEnterReadyResponse {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::runtime::UpbTypeConversions for PayloadPlayerZoneEnterReadyResponse {
    fn upb_type() -> ::protobuf::__internal::runtime::CType {
        ::protobuf::__internal::runtime::CType::Message
    }

    fn to_message_value(
        val: ::protobuf::View<'_, Self>) -> ::protobuf::__internal::runtime::upb_MessageValue {
        ::protobuf::__internal::runtime::upb_MessageValue { msg_val: Some(val.raw_msg()) }
    }

    unsafe fn into_message_value_fuse_if_required(
      raw_parent_arena: ::protobuf::__internal::runtime::RawArena,
      mut val: Self) -> ::protobuf::__internal::runtime::upb_MessageValue {
      // SAFETY: The arena memory is not freed due to `ManuallyDrop`.
      let parent_arena = ::std::mem::ManuallyDrop::new(
          unsafe { ::protobuf::__internal::runtime::Arena::from_raw(raw_parent_arena) });

      parent_arena.fuse(val.as_message_mut_inner(::protobuf::__internal::Private).arena());
      ::protobuf::__internal::runtime::upb_MessageValue { msg_val: Some(val.raw_msg()) }
    }

    unsafe fn from_message_value<'msg>(msg: ::protobuf::__internal::runtime::upb_MessageValue)
        -> ::protobuf::View<'msg, Self> {
        let raw = unsafe { msg.msg_val }.expect("expected present message value in map");
        let inner = unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap_raw(raw) };
        PayloadPlayerZoneEnterReadyResponseView::new(::protobuf::__internal::Private, inner)
    }

    unsafe fn from_message_mut<'msg>(msg: ::protobuf::__internal::runtime::RawMessage, arena: &'msg ::protobuf::__internal::runtime::Arena)
        -> PayloadPlayerZoneEnterReadyResponseMut<'msg> {
        let inner = unsafe { ::protobuf::__internal::runtime::MessageMutInner::<'msg, PayloadPlayerZoneEnterReadyResponse>::wrap_raw(msg, arena) };
        PayloadPlayerZoneEnterReadyResponseMut::new(::protobuf::__internal::Private, inner)
    }
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct PayloadPlayerZoneEnterReadyResponseMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, PayloadPlayerZoneEnterReadyResponse>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for PayloadPlayerZoneEnterReadyResponseMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for PayloadPlayerZoneEnterReadyResponseMut<'msg> {
  type Message = PayloadPlayerZoneEnterReadyResponse;
}

impl ::std::fmt::Debug for PayloadPlayerZoneEnterReadyResponseMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    let string = unsafe {
      ::protobuf::__internal::runtime::debug_string(
        self.raw_msg(),
        <Self as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table()
      )
    };
    write!(f, "{}", string)
  }
}

impl ::protobuf::Serialize for PayloadPlayerZoneEnterReadyResponseMut<'_> {
  fn serialize(&self) -> ::std::result::Result<Vec<u8>, ::protobuf::SerializeError> {
    ::protobuf::AsView::as_view(self).serialize()
  }
}

#[allow(dead_code)]
impl<'msg> PayloadPlayerZoneEnterReadyResponseMut<'msg> {
  #[doc(hidden)]
  pub fn from_parent<ParentT: ::protobuf::Message>(
             _private: ::protobuf::__internal::Private,
             parent: ::protobuf::__internal::runtime::MessageMutInner<'msg, ParentT>,
             msg: ::protobuf::__internal::runtime::RawMessage)
    -> Self {
    Self {
      inner: ::protobuf::__internal::runtime::MessageMutInner::from_parent(parent, msg)
    }
  }

  #[doc(hidden)]
  pub fn new(_private: ::protobuf::__internal::Private, inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, PayloadPlayerZoneEnterReadyResponse>) -> Self {
    Self { inner }
  }

  fn raw_msg(&self) -> ::protobuf::__internal::runtime::RawMessage {
    self.inner.raw()
  }

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, PayloadPlayerZoneEnterReadyResponse> {
    self.inner
  }

  pub fn to_owned(&self) -> PayloadPlayerZoneEnterReadyResponse {
    ::protobuf::AsView::as_view(self).to_owned()
  }

  fn arena(&mut self) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }

  // player_data: optional message Proto.RoutePlayerData
  pub fn has_player_data(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn clear_player_data(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        0
      );
    }
  }
  pub fn player_data_opt(&self) -> ::protobuf::Optional<super::RoutePlayerDataView<'_>> {
        ::protobuf::Optional::new(self.player_data(), self.has_player_data())
  }
  pub fn player_data(&self) -> super::RoutePlayerDataView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(0)
    };
    let raw = submsg.map(|ptr| ptr.raw()).unwrap_or(::protobuf::__internal::runtime::ScratchSpace::zeroed_block());
    let inner = unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap_raw(raw) };
    super::RoutePlayerDataView::new(::protobuf::__internal::Private, inner)
  }
  pub fn player_data_mut(&mut self) -> super::RoutePlayerDataMut<'_> {
     let ptr = unsafe {
       self.inner.ptr_mut().get_or_create_mutable_message_at_index(
         0, self.arena()
       ).unwrap()
     };
     super::RoutePlayerDataMut::from_parent(
       ::protobuf::__internal::Private,
       self.as_message_mut_inner(::protobuf::__internal::Private),
       ptr.raw())
  }
  pub fn set_player_data(&mut self,
    val: impl ::protobuf::IntoProxied<super::RoutePlayerData>) {

    // The message and arena are dropped after the setter. The
    // memory remains allocated as we fuse the arena with the
    // parent message's arena.
    let mut child = val.into_proxied(::protobuf::__internal::Private);
    self.inner
      .arena()
      .fuse(::protobuf::__internal::runtime::UpbGetArena::get_arena(&mut child, ::protobuf::__internal::Private));

    let child_ptr = ::protobuf::__internal::runtime::UpbGetMessagePtrMut::get_ptr_mut(&mut child, ::protobuf::__internal::Private);
    unsafe {
      self.inner.ptr_mut().set_base_field_message_at_index(
        0, child_ptr
      );
    }
  }

}

// SAFETY:
// - `PayloadPlayerZoneEnterReadyResponseMut` does not perform any shared mutation.
// - `PayloadPlayerZoneEnterReadyResponseMut` is not `Send`, and so even in the presence of mutator
//   splitting, synchronous access of an arena is impossible.
unsafe impl Sync for PayloadPlayerZoneEnterReadyResponseMut<'_> {}

impl<'msg> ::protobuf::Proxy<'msg> for PayloadPlayerZoneEnterReadyResponseMut<'msg> {}
impl<'msg> ::protobuf::MutProxy<'msg> for PayloadPlayerZoneEnterReadyResponseMut<'msg> {}

impl<'msg> ::protobuf::AsView for PayloadPlayerZoneEnterReadyResponseMut<'msg> {
  type Proxied = PayloadPlayerZoneEnterReadyResponse;
  fn as_view(&self) -> ::protobuf::View<'_, PayloadPlayerZoneEnterReadyResponse> {
    PayloadPlayerZoneEnterReadyResponseView {
      inner: ::protobuf::__internal::runtime::MessageViewInner::view_of_mut(self.inner.clone()),
      _phantom: ::std::marker::PhantomData
    }
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for PayloadPlayerZoneEnterReadyResponseMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, PayloadPlayerZoneEnterReadyResponse>
  where
      'msg: 'shorter {
    PayloadPlayerZoneEnterReadyResponseView {
      inner: ::protobuf::__internal::runtime::MessageViewInner::view_of_mut(self.inner.clone()),
      _phantom: ::std::marker::PhantomData
    }
  }
}

impl<'msg> ::protobuf::AsMut for PayloadPlayerZoneEnterReadyResponseMut<'msg> {
  type MutProxied = PayloadPlayerZoneEnterReadyResponse;
  fn as_mut(&mut self) -> PayloadPlayerZoneEnterReadyResponseMut<'msg> {
    PayloadPlayerZoneEnterReadyResponseMut { inner: self.inner }
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for PayloadPlayerZoneEnterReadyResponseMut<'msg> {
  fn into_mut<'shorter>(self) -> PayloadPlayerZoneEnterReadyResponseMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl PayloadPlayerZoneEnterReadyResponse {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }

  fn raw_msg(&self) -> ::protobuf::__internal::runtime::RawMessage {
    self.inner.raw()
  }

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, PayloadPlayerZoneEnterReadyResponse> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
  }

  fn arena(&mut self) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }

  pub fn parse(data: &[u8]) -> ::std::result::Result<Self, ::protobuf::ParseError> {
    let mut msg = Self::new();
    ::protobuf::ClearAndParse::clear_and_parse(&mut msg, data).map(|_| msg)
  }

  pub fn parse_dont_enforce_required(data: &[u8]) -> ::std::result::Result<Self, ::protobuf::ParseError> {
    let mut msg = Self::new();
    ::protobuf::ClearAndParse::clear_and_parse_dont_enforce_required(&mut msg, data).map(|_| msg)
  }

  pub fn as_view(&self) -> PayloadPlayerZoneEnterReadyResponseView {
    PayloadPlayerZoneEnterReadyResponseView::new(
        ::protobuf::__internal::Private,
        ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner))
  }

  pub fn as_mut(&mut self) -> PayloadPlayerZoneEnterReadyResponseMut {
    let inner = ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner);
    PayloadPlayerZoneEnterReadyResponseMut::new(::protobuf::__internal::Private, inner)
  }

  // player_data: optional message Proto.RoutePlayerData
  pub fn has_player_data(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn clear_player_data(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        0
      );
    }
  }
  pub fn player_data_opt(&self) -> ::protobuf::Optional<super::RoutePlayerDataView<'_>> {
        ::protobuf::Optional::new(self.player_data(), self.has_player_data())
  }
  pub fn player_data(&self) -> super::RoutePlayerDataView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(0)
    };
    let raw = submsg.map(|ptr| ptr.raw()).unwrap_or(::protobuf::__internal::runtime::ScratchSpace::zeroed_block());
    let inner = unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap_raw(raw) };
    super::RoutePlayerDataView::new(::protobuf::__internal::Private, inner)
  }
  pub fn player_data_mut(&mut self) -> super::RoutePlayerDataMut<'_> {
     let ptr = unsafe {
       self.inner.ptr_mut().get_or_create_mutable_message_at_index(
         0, self.arena()
       ).unwrap()
     };
     super::RoutePlayerDataMut::from_parent(
       ::protobuf::__internal::Private,
       self.as_message_mut_inner(::protobuf::__internal::Private),
       ptr.raw())
  }
  pub fn set_player_data(&mut self,
    val: impl ::protobuf::IntoProxied<super::RoutePlayerData>) {

    // The message and arena are dropped after the setter. The
    // memory remains allocated as we fuse the arena with the
    // parent message's arena.
    let mut child = val.into_proxied(::protobuf::__internal::Private);
    self.inner
      .arena()
      .fuse(::protobuf::__internal::runtime::UpbGetArena::get_arena(&mut child, ::protobuf::__internal::Private));

    let child_ptr = ::protobuf::__internal::runtime::UpbGetMessagePtrMut::get_ptr_mut(&mut child, ::protobuf::__internal::Private);
    unsafe {
      self.inner.ptr_mut().set_base_field_message_at_index(
        0, child_ptr
      );
    }
  }

}  // impl PayloadPlayerZoneEnterReadyResponse

impl ::std::ops::Drop for PayloadPlayerZoneEnterReadyResponse {
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for PayloadPlayerZoneEnterReadyResponse {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for PayloadPlayerZoneEnterReadyResponse {
  type Proxied = Self;
  fn as_view(&self) -> PayloadPlayerZoneEnterReadyResponseView {
    self.as_view()
  }
}

impl ::protobuf::AsMut for PayloadPlayerZoneEnterReadyResponse {
  type MutProxied = Self;
  fn as_mut(&mut self) -> PayloadPlayerZoneEnterReadyResponseMut {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for PayloadPlayerZoneEnterReadyResponse {
  fn mini_table() -> *const ::protobuf::__internal::runtime::upb_MiniTable {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTablePtr> =
        ::std::sync::OnceLock::new();
    ONCE_LOCK.get_or_init(|| unsafe {
      super::Proto__PayloadPlayerZoneEnterReadyResponse_msg_init.0 =
          ::protobuf::__internal::runtime::upb_MiniTable_Build(
              "$3".as_ptr(),
              2,
              ::protobuf::__internal::runtime::THREAD_LOCAL_ARENA.with(|a| a.raw()),
              ::std::ptr::null_mut());
      let submessages = [
        <super::RoutePlayerData as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
      ];
      let subenums = [
      ];
      assert!(::protobuf::__internal::runtime::upb_MiniTable_Link(
          super::Proto__PayloadPlayerZoneEnterReadyResponse_msg_init.0,
          submessages.as_ptr() as *const *const ::protobuf::__internal::runtime::upb_MiniTable,
          submessages.len(), subenums.as_ptr(), subenums.len()));
      ::protobuf::__internal::runtime::MiniTablePtr(super::Proto__PayloadPlayerZoneEnterReadyResponse_msg_init.0)
    }).0
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for PayloadPlayerZoneEnterReadyResponse {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for PayloadPlayerZoneEnterReadyResponseView<'_> {
  #[inline(always)]
  fn mini_table() -> *const ::protobuf::__internal::runtime::upb_MiniTable {
    <PayloadPlayerZoneEnterReadyResponse as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for PayloadPlayerZoneEnterReadyResponseMut<'_> {
  #[inline(always)]
  fn mini_table() -> *const ::protobuf::__internal::runtime::upb_MiniTable {
    <PayloadPlayerZoneEnterReadyResponse as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for PayloadPlayerZoneEnterReadyResponse {
  type Msg = PayloadPlayerZoneEnterReadyResponse;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<PayloadPlayerZoneEnterReadyResponse> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for PayloadPlayerZoneEnterReadyResponse {
  type Msg = PayloadPlayerZoneEnterReadyResponse;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<PayloadPlayerZoneEnterReadyResponse> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for PayloadPlayerZoneEnterReadyResponseMut<'_> {
  type Msg = PayloadPlayerZoneEnterReadyResponse;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<PayloadPlayerZoneEnterReadyResponse> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for PayloadPlayerZoneEnterReadyResponseMut<'_> {
  type Msg = PayloadPlayerZoneEnterReadyResponse;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<PayloadPlayerZoneEnterReadyResponse> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for PayloadPlayerZoneEnterReadyResponseView<'_> {
  type Msg = PayloadPlayerZoneEnterReadyResponse;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<PayloadPlayerZoneEnterReadyResponse> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for PayloadPlayerZoneEnterReadyResponseMut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}


// upb kernel doesn't support any owned message or message mut interop.
impl ::protobuf::OwnedMessageInterop for PayloadPlayerZoneEnterReadyResponse {}
impl<'a> ::protobuf::MessageMutInterop<'a> for PayloadPlayerZoneEnterReadyResponseMut<'a> {}

impl<'a> ::protobuf::MessageViewInterop<'a> for PayloadPlayerZoneEnterReadyResponseView<'a> {
  unsafe fn __unstable_wrap_raw_message(
    msg: &'a *const ::std::ffi::c_void) -> Self {
    let raw = ::protobuf::__internal::runtime::RawMessage::new(*msg as *mut _).unwrap();
    let inner = unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap_raw(raw) };
    Self::new(::protobuf::__internal::Private, inner)
  }
  unsafe fn __unstable_wrap_raw_message_unchecked_lifetime(
    msg: *const ::std::ffi::c_void) -> Self {
    let raw = ::protobuf::__internal::runtime::RawMessage::new(msg as *mut _).unwrap();
    let inner = unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap_raw(raw) };
    Self::new(::protobuf::__internal::Private, inner)
  }
  fn __unstable_as_raw_message(&self) -> *const ::std::ffi::c_void {
    self.inner.raw().as_ptr() as *const _
  }
}

// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut Proto__PayloadPlayerExitZoneBegin_msg_init: ::protobuf::__internal::runtime::MiniTablePtr =
    ::protobuf::__internal::runtime::MiniTablePtr(::std::ptr::null_mut());
#[allow(non_camel_case_types)]
pub struct PayloadPlayerExitZoneBegin {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<PayloadPlayerExitZoneBegin>
}

impl ::protobuf::Message for PayloadPlayerExitZoneBegin {}

impl ::std::default::Default for PayloadPlayerExitZoneBegin {
  fn default() -> Self {
    Self::new()
  }
}

impl ::protobuf::Parse for PayloadPlayerExitZoneBegin {
  fn parse(serialized: &[u8]) -> ::std::result::Result<Self, ::protobuf::ParseError> {
    Self::parse(serialized)
  }

  fn parse_dont_enforce_required(serialized: &[u8]) -> ::std::result::Result<Self, ::protobuf::ParseError> {
    Self::parse_dont_enforce_required(serialized)
  }
}

impl ::std::fmt::Debug for PayloadPlayerExitZoneBegin {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    let string = unsafe {
      ::protobuf::__internal::runtime::debug_string(
        self.raw_msg(),
        <Self as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table()
      )
    };
    write!(f, "{}", string)
  }
}

impl ::protobuf::Serialize for PayloadPlayerExitZoneBegin {
  fn serialize(&self) -> ::std::result::Result<Vec<u8>, ::protobuf::SerializeError> {
    ::protobuf::AsView::as_view(self).serialize()
  }
}

// SAFETY:
// - `PayloadPlayerExitZoneBegin` is `Sync` because it does not implement interior mutability.
//    Neither does `PayloadPlayerExitZoneBeginMut`.
unsafe impl Sync for PayloadPlayerExitZoneBegin {}

// SAFETY:
// - `PayloadPlayerExitZoneBegin` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl Send for PayloadPlayerExitZoneBegin {}

impl ::protobuf::Proxied for PayloadPlayerExitZoneBegin {
  type View<'msg> = PayloadPlayerExitZoneBeginView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for PayloadPlayerExitZoneBegin {}

impl ::protobuf::MutProxied for PayloadPlayerExitZoneBegin {
  type Mut<'msg> = PayloadPlayerExitZoneBeginMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct PayloadPlayerExitZoneBeginView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, PayloadPlayerExitZoneBegin>,
  _phantom: ::std::marker::PhantomData<&'msg ()>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for PayloadPlayerExitZoneBeginView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for PayloadPlayerExitZoneBeginView<'msg> {
  type Message = PayloadPlayerExitZoneBegin;
}

impl ::std::fmt::Debug for PayloadPlayerExitZoneBeginView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    let string = unsafe {
      ::protobuf::__internal::runtime::debug_string(
        self.raw_msg(),
        <Self as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table()
      )
    };
    write!(f, "{}", string)
  }
}

impl ::protobuf::Serialize for PayloadPlayerExitZoneBeginView<'_> {
  fn serialize(&self) -> ::std::result::Result<Vec<u8>, ::protobuf::SerializeError> {
    // SAFETY: `MINI_TABLE` is the one associated with `self.raw_msg()`.
    let encoded = unsafe {
      ::protobuf::__internal::runtime::wire::encode(self.raw_msg(),
          <Self as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table())
    };
    encoded.map_err(|_| ::protobuf::SerializeError)
  }
}

impl ::std::default::Default for PayloadPlayerExitZoneBeginView<'_> {
  fn default() -> PayloadPlayerExitZoneBeginView<'static> {
    let inner = unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap_raw(::protobuf::__internal::runtime::ScratchSpace::zeroed_block()) };
    PayloadPlayerExitZoneBeginView::new(::protobuf::__internal::Private, inner)
  }
}

#[allow(dead_code)]
impl<'msg> PayloadPlayerExitZoneBeginView<'msg> {
  #[doc(hidden)]
  pub fn new(_private: ::protobuf::__internal::Private, inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, PayloadPlayerExitZoneBegin>) -> Self {
    Self { inner, _phantom: ::std::marker::PhantomData }
  }

  fn raw_msg(&self) -> ::protobuf::__internal::runtime::RawMessage {
    self.inner.raw()
  }

  pub fn to_owned(&self) -> PayloadPlayerExitZoneBegin {
    ::protobuf::IntoProxied::into_proxied(*self, ::protobuf::__internal::Private)
  }

  // user_id: optional uint64
  pub fn user_id(self) -> u64 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_u64_at_index(
        0, (0u64).into()
      ).try_into().unwrap()
    }
  }

}

// SAFETY:
// - `PayloadPlayerExitZoneBeginView` is `Sync` because it does not support mutation.
unsafe impl Sync for PayloadPlayerExitZoneBeginView<'_> {}

// SAFETY:
// - `PayloadPlayerExitZoneBeginView` is `Send` because while its alive a `PayloadPlayerExitZoneBeginMut` cannot.
// - `PayloadPlayerExitZoneBeginView` does not use thread-local data.
unsafe impl Send for PayloadPlayerExitZoneBeginView<'_> {}

impl<'msg> ::protobuf::Proxy<'msg> for PayloadPlayerExitZoneBeginView<'msg> {}
impl<'msg> ::protobuf::ViewProxy<'msg> for PayloadPlayerExitZoneBeginView<'msg> {}

impl<'msg> ::protobuf::AsView for PayloadPlayerExitZoneBeginView<'msg> {
  type Proxied = PayloadPlayerExitZoneBegin;
  fn as_view(&self) -> ::protobuf::View<'msg, PayloadPlayerExitZoneBegin> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for PayloadPlayerExitZoneBeginView<'msg> {
  fn into_view<'shorter>(self) -> PayloadPlayerExitZoneBeginView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<PayloadPlayerExitZoneBegin> for PayloadPlayerExitZoneBeginView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> PayloadPlayerExitZoneBegin {
    let mut dst = PayloadPlayerExitZoneBegin::new();
    let dst_raw = ::protobuf::__internal::runtime::UpbGetMessagePtrMut::get_raw_message_mut(&mut dst, ::protobuf::__internal::Private);
    let dst_arena = ::protobuf::__internal::runtime::UpbGetArena::get_arena(&mut dst, ::protobuf::__internal::Private);
    let src_raw = ::protobuf::__internal::runtime::UpbGetMessagePtr::get_raw_message(&self, ::protobuf::__internal::Private);

    unsafe { ::protobuf::__internal::runtime::upb_Message_DeepCopy(
      dst_raw,
      src_raw,
      <Self as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
      dst_arena.raw(),
    ) };
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<PayloadPlayerExitZoneBegin> for PayloadPlayerExitZoneBeginMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> PayloadPlayerExitZoneBegin {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::runtime::UpbTypeConversions for PayloadPlayerExitZoneBegin {
    fn upb_type() -> ::protobuf::__internal::runtime::CType {
        ::protobuf::__internal::runtime::CType::Message
    }

    fn to_message_value(
        val: ::protobuf::View<'_, Self>) -> ::protobuf::__internal::runtime::upb_MessageValue {
        ::protobuf::__internal::runtime::upb_MessageValue { msg_val: Some(val.raw_msg()) }
    }

    unsafe fn into_message_value_fuse_if_required(
      raw_parent_arena: ::protobuf::__internal::runtime::RawArena,
      mut val: Self) -> ::protobuf::__internal::runtime::upb_MessageValue {
      // SAFETY: The arena memory is not freed due to `ManuallyDrop`.
      let parent_arena = ::std::mem::ManuallyDrop::new(
          unsafe { ::protobuf::__internal::runtime::Arena::from_raw(raw_parent_arena) });

      parent_arena.fuse(val.as_message_mut_inner(::protobuf::__internal::Private).arena());
      ::protobuf::__internal::runtime::upb_MessageValue { msg_val: Some(val.raw_msg()) }
    }

    unsafe fn from_message_value<'msg>(msg: ::protobuf::__internal::runtime::upb_MessageValue)
        -> ::protobuf::View<'msg, Self> {
        let raw = unsafe { msg.msg_val }.expect("expected present message value in map");
        let inner = unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap_raw(raw) };
        PayloadPlayerExitZoneBeginView::new(::protobuf::__internal::Private, inner)
    }

    unsafe fn from_message_mut<'msg>(msg: ::protobuf::__internal::runtime::RawMessage, arena: &'msg ::protobuf::__internal::runtime::Arena)
        -> PayloadPlayerExitZoneBeginMut<'msg> {
        let inner = unsafe { ::protobuf::__internal::runtime::MessageMutInner::<'msg, PayloadPlayerExitZoneBegin>::wrap_raw(msg, arena) };
        PayloadPlayerExitZoneBeginMut::new(::protobuf::__internal::Private, inner)
    }
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct PayloadPlayerExitZoneBeginMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, PayloadPlayerExitZoneBegin>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for PayloadPlayerExitZoneBeginMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for PayloadPlayerExitZoneBeginMut<'msg> {
  type Message = PayloadPlayerExitZoneBegin;
}

impl ::std::fmt::Debug for PayloadPlayerExitZoneBeginMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    let string = unsafe {
      ::protobuf::__internal::runtime::debug_string(
        self.raw_msg(),
        <Self as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table()
      )
    };
    write!(f, "{}", string)
  }
}

impl ::protobuf::Serialize for PayloadPlayerExitZoneBeginMut<'_> {
  fn serialize(&self) -> ::std::result::Result<Vec<u8>, ::protobuf::SerializeError> {
    ::protobuf::AsView::as_view(self).serialize()
  }
}

#[allow(dead_code)]
impl<'msg> PayloadPlayerExitZoneBeginMut<'msg> {
  #[doc(hidden)]
  pub fn from_parent<ParentT: ::protobuf::Message>(
             _private: ::protobuf::__internal::Private,
             parent: ::protobuf::__internal::runtime::MessageMutInner<'msg, ParentT>,
             msg: ::protobuf::__internal::runtime::RawMessage)
    -> Self {
    Self {
      inner: ::protobuf::__internal::runtime::MessageMutInner::from_parent(parent, msg)
    }
  }

  #[doc(hidden)]
  pub fn new(_private: ::protobuf::__internal::Private, inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, PayloadPlayerExitZoneBegin>) -> Self {
    Self { inner }
  }

  fn raw_msg(&self) -> ::protobuf::__internal::runtime::RawMessage {
    self.inner.raw()
  }

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, PayloadPlayerExitZoneBegin> {
    self.inner
  }

  pub fn to_owned(&self) -> PayloadPlayerExitZoneBegin {
    ::protobuf::AsView::as_view(self).to_owned()
  }

  fn arena(&mut self) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }

  // user_id: optional uint64
  pub fn user_id(&self) -> u64 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_u64_at_index(
        0, (0u64).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_user_id(&mut self, val: u64) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_u64_at_index(
        0, val.into()
      )
    }
  }

}

// SAFETY:
// - `PayloadPlayerExitZoneBeginMut` does not perform any shared mutation.
// - `PayloadPlayerExitZoneBeginMut` is not `Send`, and so even in the presence of mutator
//   splitting, synchronous access of an arena is impossible.
unsafe impl Sync for PayloadPlayerExitZoneBeginMut<'_> {}

impl<'msg> ::protobuf::Proxy<'msg> for PayloadPlayerExitZoneBeginMut<'msg> {}
impl<'msg> ::protobuf::MutProxy<'msg> for PayloadPlayerExitZoneBeginMut<'msg> {}

impl<'msg> ::protobuf::AsView for PayloadPlayerExitZoneBeginMut<'msg> {
  type Proxied = PayloadPlayerExitZoneBegin;
  fn as_view(&self) -> ::protobuf::View<'_, PayloadPlayerExitZoneBegin> {
    PayloadPlayerExitZoneBeginView {
      inner: ::protobuf::__internal::runtime::MessageViewInner::view_of_mut(self.inner.clone()),
      _phantom: ::std::marker::PhantomData
    }
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for PayloadPlayerExitZoneBeginMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, PayloadPlayerExitZoneBegin>
  where
      'msg: 'shorter {
    PayloadPlayerExitZoneBeginView {
      inner: ::protobuf::__internal::runtime::MessageViewInner::view_of_mut(self.inner.clone()),
      _phantom: ::std::marker::PhantomData
    }
  }
}

impl<'msg> ::protobuf::AsMut for PayloadPlayerExitZoneBeginMut<'msg> {
  type MutProxied = PayloadPlayerExitZoneBegin;
  fn as_mut(&mut self) -> PayloadPlayerExitZoneBeginMut<'msg> {
    PayloadPlayerExitZoneBeginMut { inner: self.inner }
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for PayloadPlayerExitZoneBeginMut<'msg> {
  fn into_mut<'shorter>(self) -> PayloadPlayerExitZoneBeginMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl PayloadPlayerExitZoneBegin {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }

  fn raw_msg(&self) -> ::protobuf::__internal::runtime::RawMessage {
    self.inner.raw()
  }

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, PayloadPlayerExitZoneBegin> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
  }

  fn arena(&mut self) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }

  pub fn parse(data: &[u8]) -> ::std::result::Result<Self, ::protobuf::ParseError> {
    let mut msg = Self::new();
    ::protobuf::ClearAndParse::clear_and_parse(&mut msg, data).map(|_| msg)
  }

  pub fn parse_dont_enforce_required(data: &[u8]) -> ::std::result::Result<Self, ::protobuf::ParseError> {
    let mut msg = Self::new();
    ::protobuf::ClearAndParse::clear_and_parse_dont_enforce_required(&mut msg, data).map(|_| msg)
  }

  pub fn as_view(&self) -> PayloadPlayerExitZoneBeginView {
    PayloadPlayerExitZoneBeginView::new(
        ::protobuf::__internal::Private,
        ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner))
  }

  pub fn as_mut(&mut self) -> PayloadPlayerExitZoneBeginMut {
    let inner = ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner);
    PayloadPlayerExitZoneBeginMut::new(::protobuf::__internal::Private, inner)
  }

  // user_id: optional uint64
  pub fn user_id(&self) -> u64 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_u64_at_index(
        0, (0u64).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_user_id(&mut self, val: u64) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_u64_at_index(
        0, val.into()
      )
    }
  }

}  // impl PayloadPlayerExitZoneBegin

impl ::std::ops::Drop for PayloadPlayerExitZoneBegin {
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for PayloadPlayerExitZoneBegin {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for PayloadPlayerExitZoneBegin {
  type Proxied = Self;
  fn as_view(&self) -> PayloadPlayerExitZoneBeginView {
    self.as_view()
  }
}

impl ::protobuf::AsMut for PayloadPlayerExitZoneBegin {
  type MutProxied = Self;
  fn as_mut(&mut self) -> PayloadPlayerExitZoneBeginMut {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for PayloadPlayerExitZoneBegin {
  fn mini_table() -> *const ::protobuf::__internal::runtime::upb_MiniTable {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTablePtr> =
        ::std::sync::OnceLock::new();
    ONCE_LOCK.get_or_init(|| unsafe {
      super::Proto__PayloadPlayerExitZoneBegin_msg_init.0 =
          ::protobuf::__internal::runtime::upb_MiniTable_Build(
              "$,P".as_ptr(),
              3,
              ::protobuf::__internal::runtime::THREAD_LOCAL_ARENA.with(|a| a.raw()),
              ::std::ptr::null_mut());
      let submessages = [
      ];
      let subenums = [
      ];
      assert!(::protobuf::__internal::runtime::upb_MiniTable_Link(
          super::Proto__PayloadPlayerExitZoneBegin_msg_init.0,
          submessages.as_ptr() as *const *const ::protobuf::__internal::runtime::upb_MiniTable,
          submessages.len(), subenums.as_ptr(), subenums.len()));
      ::protobuf::__internal::runtime::MiniTablePtr(super::Proto__PayloadPlayerExitZoneBegin_msg_init.0)
    }).0
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for PayloadPlayerExitZoneBegin {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for PayloadPlayerExitZoneBeginView<'_> {
  #[inline(always)]
  fn mini_table() -> *const ::protobuf::__internal::runtime::upb_MiniTable {
    <PayloadPlayerExitZoneBegin as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for PayloadPlayerExitZoneBeginMut<'_> {
  #[inline(always)]
  fn mini_table() -> *const ::protobuf::__internal::runtime::upb_MiniTable {
    <PayloadPlayerExitZoneBegin as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for PayloadPlayerExitZoneBegin {
  type Msg = PayloadPlayerExitZoneBegin;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<PayloadPlayerExitZoneBegin> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for PayloadPlayerExitZoneBegin {
  type Msg = PayloadPlayerExitZoneBegin;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<PayloadPlayerExitZoneBegin> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for PayloadPlayerExitZoneBeginMut<'_> {
  type Msg = PayloadPlayerExitZoneBegin;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<PayloadPlayerExitZoneBegin> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for PayloadPlayerExitZoneBeginMut<'_> {
  type Msg = PayloadPlayerExitZoneBegin;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<PayloadPlayerExitZoneBegin> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for PayloadPlayerExitZoneBeginView<'_> {
  type Msg = PayloadPlayerExitZoneBegin;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<PayloadPlayerExitZoneBegin> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for PayloadPlayerExitZoneBeginMut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}


// upb kernel doesn't support any owned message or message mut interop.
impl ::protobuf::OwnedMessageInterop for PayloadPlayerExitZoneBegin {}
impl<'a> ::protobuf::MessageMutInterop<'a> for PayloadPlayerExitZoneBeginMut<'a> {}

impl<'a> ::protobuf::MessageViewInterop<'a> for PayloadPlayerExitZoneBeginView<'a> {
  unsafe fn __unstable_wrap_raw_message(
    msg: &'a *const ::std::ffi::c_void) -> Self {
    let raw = ::protobuf::__internal::runtime::RawMessage::new(*msg as *mut _).unwrap();
    let inner = unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap_raw(raw) };
    Self::new(::protobuf::__internal::Private, inner)
  }
  unsafe fn __unstable_wrap_raw_message_unchecked_lifetime(
    msg: *const ::std::ffi::c_void) -> Self {
    let raw = ::protobuf::__internal::runtime::RawMessage::new(msg as *mut _).unwrap();
    let inner = unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap_raw(raw) };
    Self::new(::protobuf::__internal::Private, inner)
  }
  fn __unstable_as_raw_message(&self) -> *const ::std::ffi::c_void {
    self.inner.raw().as_ptr() as *const _
  }
}

// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut Proto__PayloadPlayerExitZoneReady_msg_init: ::protobuf::__internal::runtime::MiniTablePtr =
    ::protobuf::__internal::runtime::MiniTablePtr(::std::ptr::null_mut());
#[allow(non_camel_case_types)]
pub struct PayloadPlayerExitZoneReady {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<PayloadPlayerExitZoneReady>
}

impl ::protobuf::Message for PayloadPlayerExitZoneReady {}

impl ::std::default::Default for PayloadPlayerExitZoneReady {
  fn default() -> Self {
    Self::new()
  }
}

impl ::protobuf::Parse for PayloadPlayerExitZoneReady {
  fn parse(serialized: &[u8]) -> ::std::result::Result<Self, ::protobuf::ParseError> {
    Self::parse(serialized)
  }

  fn parse_dont_enforce_required(serialized: &[u8]) -> ::std::result::Result<Self, ::protobuf::ParseError> {
    Self::parse_dont_enforce_required(serialized)
  }
}

impl ::std::fmt::Debug for PayloadPlayerExitZoneReady {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    let string = unsafe {
      ::protobuf::__internal::runtime::debug_string(
        self.raw_msg(),
        <Self as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table()
      )
    };
    write!(f, "{}", string)
  }
}

impl ::protobuf::Serialize for PayloadPlayerExitZoneReady {
  fn serialize(&self) -> ::std::result::Result<Vec<u8>, ::protobuf::SerializeError> {
    ::protobuf::AsView::as_view(self).serialize()
  }
}

// SAFETY:
// - `PayloadPlayerExitZoneReady` is `Sync` because it does not implement interior mutability.
//    Neither does `PayloadPlayerExitZoneReadyMut`.
unsafe impl Sync for PayloadPlayerExitZoneReady {}

// SAFETY:
// - `PayloadPlayerExitZoneReady` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl Send for PayloadPlayerExitZoneReady {}

impl ::protobuf::Proxied for PayloadPlayerExitZoneReady {
  type View<'msg> = PayloadPlayerExitZoneReadyView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for PayloadPlayerExitZoneReady {}

impl ::protobuf::MutProxied for PayloadPlayerExitZoneReady {
  type Mut<'msg> = PayloadPlayerExitZoneReadyMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct PayloadPlayerExitZoneReadyView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, PayloadPlayerExitZoneReady>,
  _phantom: ::std::marker::PhantomData<&'msg ()>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for PayloadPlayerExitZoneReadyView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for PayloadPlayerExitZoneReadyView<'msg> {
  type Message = PayloadPlayerExitZoneReady;
}

impl ::std::fmt::Debug for PayloadPlayerExitZoneReadyView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    let string = unsafe {
      ::protobuf::__internal::runtime::debug_string(
        self.raw_msg(),
        <Self as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table()
      )
    };
    write!(f, "{}", string)
  }
}

impl ::protobuf::Serialize for PayloadPlayerExitZoneReadyView<'_> {
  fn serialize(&self) -> ::std::result::Result<Vec<u8>, ::protobuf::SerializeError> {
    // SAFETY: `MINI_TABLE` is the one associated with `self.raw_msg()`.
    let encoded = unsafe {
      ::protobuf::__internal::runtime::wire::encode(self.raw_msg(),
          <Self as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table())
    };
    encoded.map_err(|_| ::protobuf::SerializeError)
  }
}

impl ::std::default::Default for PayloadPlayerExitZoneReadyView<'_> {
  fn default() -> PayloadPlayerExitZoneReadyView<'static> {
    let inner = unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap_raw(::protobuf::__internal::runtime::ScratchSpace::zeroed_block()) };
    PayloadPlayerExitZoneReadyView::new(::protobuf::__internal::Private, inner)
  }
}

#[allow(dead_code)]
impl<'msg> PayloadPlayerExitZoneReadyView<'msg> {
  #[doc(hidden)]
  pub fn new(_private: ::protobuf::__internal::Private, inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, PayloadPlayerExitZoneReady>) -> Self {
    Self { inner, _phantom: ::std::marker::PhantomData }
  }

  fn raw_msg(&self) -> ::protobuf::__internal::runtime::RawMessage {
    self.inner.raw()
  }

  pub fn to_owned(&self) -> PayloadPlayerExitZoneReady {
    ::protobuf::IntoProxied::into_proxied(*self, ::protobuf::__internal::Private)
  }

  // user_id: optional uint64
  pub fn user_id(self) -> u64 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_u64_at_index(
        0, (0u64).into()
      ).try_into().unwrap()
    }
  }

}

// SAFETY:
// - `PayloadPlayerExitZoneReadyView` is `Sync` because it does not support mutation.
unsafe impl Sync for PayloadPlayerExitZoneReadyView<'_> {}

// SAFETY:
// - `PayloadPlayerExitZoneReadyView` is `Send` because while its alive a `PayloadPlayerExitZoneReadyMut` cannot.
// - `PayloadPlayerExitZoneReadyView` does not use thread-local data.
unsafe impl Send for PayloadPlayerExitZoneReadyView<'_> {}

impl<'msg> ::protobuf::Proxy<'msg> for PayloadPlayerExitZoneReadyView<'msg> {}
impl<'msg> ::protobuf::ViewProxy<'msg> for PayloadPlayerExitZoneReadyView<'msg> {}

impl<'msg> ::protobuf::AsView for PayloadPlayerExitZoneReadyView<'msg> {
  type Proxied = PayloadPlayerExitZoneReady;
  fn as_view(&self) -> ::protobuf::View<'msg, PayloadPlayerExitZoneReady> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for PayloadPlayerExitZoneReadyView<'msg> {
  fn into_view<'shorter>(self) -> PayloadPlayerExitZoneReadyView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<PayloadPlayerExitZoneReady> for PayloadPlayerExitZoneReadyView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> PayloadPlayerExitZoneReady {
    let mut dst = PayloadPlayerExitZoneReady::new();
    let dst_raw = ::protobuf::__internal::runtime::UpbGetMessagePtrMut::get_raw_message_mut(&mut dst, ::protobuf::__internal::Private);
    let dst_arena = ::protobuf::__internal::runtime::UpbGetArena::get_arena(&mut dst, ::protobuf::__internal::Private);
    let src_raw = ::protobuf::__internal::runtime::UpbGetMessagePtr::get_raw_message(&self, ::protobuf::__internal::Private);

    unsafe { ::protobuf::__internal::runtime::upb_Message_DeepCopy(
      dst_raw,
      src_raw,
      <Self as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
      dst_arena.raw(),
    ) };
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<PayloadPlayerExitZoneReady> for PayloadPlayerExitZoneReadyMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> PayloadPlayerExitZoneReady {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::runtime::UpbTypeConversions for PayloadPlayerExitZoneReady {
    fn upb_type() -> ::protobuf::__internal::runtime::CType {
        ::protobuf::__internal::runtime::CType::Message
    }

    fn to_message_value(
        val: ::protobuf::View<'_, Self>) -> ::protobuf::__internal::runtime::upb_MessageValue {
        ::protobuf::__internal::runtime::upb_MessageValue { msg_val: Some(val.raw_msg()) }
    }

    unsafe fn into_message_value_fuse_if_required(
      raw_parent_arena: ::protobuf::__internal::runtime::RawArena,
      mut val: Self) -> ::protobuf::__internal::runtime::upb_MessageValue {
      // SAFETY: The arena memory is not freed due to `ManuallyDrop`.
      let parent_arena = ::std::mem::ManuallyDrop::new(
          unsafe { ::protobuf::__internal::runtime::Arena::from_raw(raw_parent_arena) });

      parent_arena.fuse(val.as_message_mut_inner(::protobuf::__internal::Private).arena());
      ::protobuf::__internal::runtime::upb_MessageValue { msg_val: Some(val.raw_msg()) }
    }

    unsafe fn from_message_value<'msg>(msg: ::protobuf::__internal::runtime::upb_MessageValue)
        -> ::protobuf::View<'msg, Self> {
        let raw = unsafe { msg.msg_val }.expect("expected present message value in map");
        let inner = unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap_raw(raw) };
        PayloadPlayerExitZoneReadyView::new(::protobuf::__internal::Private, inner)
    }

    unsafe fn from_message_mut<'msg>(msg: ::protobuf::__internal::runtime::RawMessage, arena: &'msg ::protobuf::__internal::runtime::Arena)
        -> PayloadPlayerExitZoneReadyMut<'msg> {
        let inner = unsafe { ::protobuf::__internal::runtime::MessageMutInner::<'msg, PayloadPlayerExitZoneReady>::wrap_raw(msg, arena) };
        PayloadPlayerExitZoneReadyMut::new(::protobuf::__internal::Private, inner)
    }
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct PayloadPlayerExitZoneReadyMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, PayloadPlayerExitZoneReady>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for PayloadPlayerExitZoneReadyMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for PayloadPlayerExitZoneReadyMut<'msg> {
  type Message = PayloadPlayerExitZoneReady;
}

impl ::std::fmt::Debug for PayloadPlayerExitZoneReadyMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    let string = unsafe {
      ::protobuf::__internal::runtime::debug_string(
        self.raw_msg(),
        <Self as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table()
      )
    };
    write!(f, "{}", string)
  }
}

impl ::protobuf::Serialize for PayloadPlayerExitZoneReadyMut<'_> {
  fn serialize(&self) -> ::std::result::Result<Vec<u8>, ::protobuf::SerializeError> {
    ::protobuf::AsView::as_view(self).serialize()
  }
}

#[allow(dead_code)]
impl<'msg> PayloadPlayerExitZoneReadyMut<'msg> {
  #[doc(hidden)]
  pub fn from_parent<ParentT: ::protobuf::Message>(
             _private: ::protobuf::__internal::Private,
             parent: ::protobuf::__internal::runtime::MessageMutInner<'msg, ParentT>,
             msg: ::protobuf::__internal::runtime::RawMessage)
    -> Self {
    Self {
      inner: ::protobuf::__internal::runtime::MessageMutInner::from_parent(parent, msg)
    }
  }

  #[doc(hidden)]
  pub fn new(_private: ::protobuf::__internal::Private, inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, PayloadPlayerExitZoneReady>) -> Self {
    Self { inner }
  }

  fn raw_msg(&self) -> ::protobuf::__internal::runtime::RawMessage {
    self.inner.raw()
  }

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, PayloadPlayerExitZoneReady> {
    self.inner
  }

  pub fn to_owned(&self) -> PayloadPlayerExitZoneReady {
    ::protobuf::AsView::as_view(self).to_owned()
  }

  fn arena(&mut self) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }

  // user_id: optional uint64
  pub fn user_id(&self) -> u64 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_u64_at_index(
        0, (0u64).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_user_id(&mut self, val: u64) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_u64_at_index(
        0, val.into()
      )
    }
  }

}

// SAFETY:
// - `PayloadPlayerExitZoneReadyMut` does not perform any shared mutation.
// - `PayloadPlayerExitZoneReadyMut` is not `Send`, and so even in the presence of mutator
//   splitting, synchronous access of an arena is impossible.
unsafe impl Sync for PayloadPlayerExitZoneReadyMut<'_> {}

impl<'msg> ::protobuf::Proxy<'msg> for PayloadPlayerExitZoneReadyMut<'msg> {}
impl<'msg> ::protobuf::MutProxy<'msg> for PayloadPlayerExitZoneReadyMut<'msg> {}

impl<'msg> ::protobuf::AsView for PayloadPlayerExitZoneReadyMut<'msg> {
  type Proxied = PayloadPlayerExitZoneReady;
  fn as_view(&self) -> ::protobuf::View<'_, PayloadPlayerExitZoneReady> {
    PayloadPlayerExitZoneReadyView {
      inner: ::protobuf::__internal::runtime::MessageViewInner::view_of_mut(self.inner.clone()),
      _phantom: ::std::marker::PhantomData
    }
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for PayloadPlayerExitZoneReadyMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, PayloadPlayerExitZoneReady>
  where
      'msg: 'shorter {
    PayloadPlayerExitZoneReadyView {
      inner: ::protobuf::__internal::runtime::MessageViewInner::view_of_mut(self.inner.clone()),
      _phantom: ::std::marker::PhantomData
    }
  }
}

impl<'msg> ::protobuf::AsMut for PayloadPlayerExitZoneReadyMut<'msg> {
  type MutProxied = PayloadPlayerExitZoneReady;
  fn as_mut(&mut self) -> PayloadPlayerExitZoneReadyMut<'msg> {
    PayloadPlayerExitZoneReadyMut { inner: self.inner }
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for PayloadPlayerExitZoneReadyMut<'msg> {
  fn into_mut<'shorter>(self) -> PayloadPlayerExitZoneReadyMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl PayloadPlayerExitZoneReady {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }

  fn raw_msg(&self) -> ::protobuf::__internal::runtime::RawMessage {
    self.inner.raw()
  }

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, PayloadPlayerExitZoneReady> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
  }

  fn arena(&mut self) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }

  pub fn parse(data: &[u8]) -> ::std::result::Result<Self, ::protobuf::ParseError> {
    let mut msg = Self::new();
    ::protobuf::ClearAndParse::clear_and_parse(&mut msg, data).map(|_| msg)
  }

  pub fn parse_dont_enforce_required(data: &[u8]) -> ::std::result::Result<Self, ::protobuf::ParseError> {
    let mut msg = Self::new();
    ::protobuf::ClearAndParse::clear_and_parse_dont_enforce_required(&mut msg, data).map(|_| msg)
  }

  pub fn as_view(&self) -> PayloadPlayerExitZoneReadyView {
    PayloadPlayerExitZoneReadyView::new(
        ::protobuf::__internal::Private,
        ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner))
  }

  pub fn as_mut(&mut self) -> PayloadPlayerExitZoneReadyMut {
    let inner = ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner);
    PayloadPlayerExitZoneReadyMut::new(::protobuf::__internal::Private, inner)
  }

  // user_id: optional uint64
  pub fn user_id(&self) -> u64 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_u64_at_index(
        0, (0u64).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_user_id(&mut self, val: u64) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_u64_at_index(
        0, val.into()
      )
    }
  }

}  // impl PayloadPlayerExitZoneReady

impl ::std::ops::Drop for PayloadPlayerExitZoneReady {
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for PayloadPlayerExitZoneReady {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for PayloadPlayerExitZoneReady {
  type Proxied = Self;
  fn as_view(&self) -> PayloadPlayerExitZoneReadyView {
    self.as_view()
  }
}

impl ::protobuf::AsMut for PayloadPlayerExitZoneReady {
  type MutProxied = Self;
  fn as_mut(&mut self) -> PayloadPlayerExitZoneReadyMut {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for PayloadPlayerExitZoneReady {
  fn mini_table() -> *const ::protobuf::__internal::runtime::upb_MiniTable {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTablePtr> =
        ::std::sync::OnceLock::new();
    ONCE_LOCK.get_or_init(|| unsafe {
      super::Proto__PayloadPlayerExitZoneReady_msg_init.0 =
          ::protobuf::__internal::runtime::upb_MiniTable_Build(
              "$,P".as_ptr(),
              3,
              ::protobuf::__internal::runtime::THREAD_LOCAL_ARENA.with(|a| a.raw()),
              ::std::ptr::null_mut());
      let submessages = [
      ];
      let subenums = [
      ];
      assert!(::protobuf::__internal::runtime::upb_MiniTable_Link(
          super::Proto__PayloadPlayerExitZoneReady_msg_init.0,
          submessages.as_ptr() as *const *const ::protobuf::__internal::runtime::upb_MiniTable,
          submessages.len(), subenums.as_ptr(), subenums.len()));
      ::protobuf::__internal::runtime::MiniTablePtr(super::Proto__PayloadPlayerExitZoneReady_msg_init.0)
    }).0
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for PayloadPlayerExitZoneReady {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for PayloadPlayerExitZoneReadyView<'_> {
  #[inline(always)]
  fn mini_table() -> *const ::protobuf::__internal::runtime::upb_MiniTable {
    <PayloadPlayerExitZoneReady as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for PayloadPlayerExitZoneReadyMut<'_> {
  #[inline(always)]
  fn mini_table() -> *const ::protobuf::__internal::runtime::upb_MiniTable {
    <PayloadPlayerExitZoneReady as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for PayloadPlayerExitZoneReady {
  type Msg = PayloadPlayerExitZoneReady;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<PayloadPlayerExitZoneReady> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for PayloadPlayerExitZoneReady {
  type Msg = PayloadPlayerExitZoneReady;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<PayloadPlayerExitZoneReady> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for PayloadPlayerExitZoneReadyMut<'_> {
  type Msg = PayloadPlayerExitZoneReady;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<PayloadPlayerExitZoneReady> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for PayloadPlayerExitZoneReadyMut<'_> {
  type Msg = PayloadPlayerExitZoneReady;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<PayloadPlayerExitZoneReady> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for PayloadPlayerExitZoneReadyView<'_> {
  type Msg = PayloadPlayerExitZoneReady;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<PayloadPlayerExitZoneReady> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for PayloadPlayerExitZoneReadyMut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}


// upb kernel doesn't support any owned message or message mut interop.
impl ::protobuf::OwnedMessageInterop for PayloadPlayerExitZoneReady {}
impl<'a> ::protobuf::MessageMutInterop<'a> for PayloadPlayerExitZoneReadyMut<'a> {}

impl<'a> ::protobuf::MessageViewInterop<'a> for PayloadPlayerExitZoneReadyView<'a> {
  unsafe fn __unstable_wrap_raw_message(
    msg: &'a *const ::std::ffi::c_void) -> Self {
    let raw = ::protobuf::__internal::runtime::RawMessage::new(*msg as *mut _).unwrap();
    let inner = unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap_raw(raw) };
    Self::new(::protobuf::__internal::Private, inner)
  }
  unsafe fn __unstable_wrap_raw_message_unchecked_lifetime(
    msg: *const ::std::ffi::c_void) -> Self {
    let raw = ::protobuf::__internal::runtime::RawMessage::new(msg as *mut _).unwrap();
    let inner = unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap_raw(raw) };
    Self::new(::protobuf::__internal::Private, inner)
  }
  fn __unstable_as_raw_message(&self) -> *const ::std::ffi::c_void {
    self.inner.raw().as_ptr() as *const _
  }
}

// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut Proto__PayloadPlayerExitZoneComplete_msg_init: ::protobuf::__internal::runtime::MiniTablePtr =
    ::protobuf::__internal::runtime::MiniTablePtr(::std::ptr::null_mut());
#[allow(non_camel_case_types)]
pub struct PayloadPlayerExitZoneComplete {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<PayloadPlayerExitZoneComplete>
}

impl ::protobuf::Message for PayloadPlayerExitZoneComplete {}

impl ::std::default::Default for PayloadPlayerExitZoneComplete {
  fn default() -> Self {
    Self::new()
  }
}

impl ::protobuf::Parse for PayloadPlayerExitZoneComplete {
  fn parse(serialized: &[u8]) -> ::std::result::Result<Self, ::protobuf::ParseError> {
    Self::parse(serialized)
  }

  fn parse_dont_enforce_required(serialized: &[u8]) -> ::std::result::Result<Self, ::protobuf::ParseError> {
    Self::parse_dont_enforce_required(serialized)
  }
}

impl ::std::fmt::Debug for PayloadPlayerExitZoneComplete {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    let string = unsafe {
      ::protobuf::__internal::runtime::debug_string(
        self.raw_msg(),
        <Self as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table()
      )
    };
    write!(f, "{}", string)
  }
}

impl ::protobuf::Serialize for PayloadPlayerExitZoneComplete {
  fn serialize(&self) -> ::std::result::Result<Vec<u8>, ::protobuf::SerializeError> {
    ::protobuf::AsView::as_view(self).serialize()
  }
}

// SAFETY:
// - `PayloadPlayerExitZoneComplete` is `Sync` because it does not implement interior mutability.
//    Neither does `PayloadPlayerExitZoneCompleteMut`.
unsafe impl Sync for PayloadPlayerExitZoneComplete {}

// SAFETY:
// - `PayloadPlayerExitZoneComplete` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl Send for PayloadPlayerExitZoneComplete {}

impl ::protobuf::Proxied for PayloadPlayerExitZoneComplete {
  type View<'msg> = PayloadPlayerExitZoneCompleteView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for PayloadPlayerExitZoneComplete {}

impl ::protobuf::MutProxied for PayloadPlayerExitZoneComplete {
  type Mut<'msg> = PayloadPlayerExitZoneCompleteMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct PayloadPlayerExitZoneCompleteView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, PayloadPlayerExitZoneComplete>,
  _phantom: ::std::marker::PhantomData<&'msg ()>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for PayloadPlayerExitZoneCompleteView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for PayloadPlayerExitZoneCompleteView<'msg> {
  type Message = PayloadPlayerExitZoneComplete;
}

impl ::std::fmt::Debug for PayloadPlayerExitZoneCompleteView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    let string = unsafe {
      ::protobuf::__internal::runtime::debug_string(
        self.raw_msg(),
        <Self as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table()
      )
    };
    write!(f, "{}", string)
  }
}

impl ::protobuf::Serialize for PayloadPlayerExitZoneCompleteView<'_> {
  fn serialize(&self) -> ::std::result::Result<Vec<u8>, ::protobuf::SerializeError> {
    // SAFETY: `MINI_TABLE` is the one associated with `self.raw_msg()`.
    let encoded = unsafe {
      ::protobuf::__internal::runtime::wire::encode(self.raw_msg(),
          <Self as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table())
    };
    encoded.map_err(|_| ::protobuf::SerializeError)
  }
}

impl ::std::default::Default for PayloadPlayerExitZoneCompleteView<'_> {
  fn default() -> PayloadPlayerExitZoneCompleteView<'static> {
    let inner = unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap_raw(::protobuf::__internal::runtime::ScratchSpace::zeroed_block()) };
    PayloadPlayerExitZoneCompleteView::new(::protobuf::__internal::Private, inner)
  }
}

#[allow(dead_code)]
impl<'msg> PayloadPlayerExitZoneCompleteView<'msg> {
  #[doc(hidden)]
  pub fn new(_private: ::protobuf::__internal::Private, inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, PayloadPlayerExitZoneComplete>) -> Self {
    Self { inner, _phantom: ::std::marker::PhantomData }
  }

  fn raw_msg(&self) -> ::protobuf::__internal::runtime::RawMessage {
    self.inner.raw()
  }

  pub fn to_owned(&self) -> PayloadPlayerExitZoneComplete {
    ::protobuf::IntoProxied::into_proxied(*self, ::protobuf::__internal::Private)
  }

  // user_id: optional uint64
  pub fn user_id(self) -> u64 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_u64_at_index(
        0, (0u64).into()
      ).try_into().unwrap()
    }
  }

}

// SAFETY:
// - `PayloadPlayerExitZoneCompleteView` is `Sync` because it does not support mutation.
unsafe impl Sync for PayloadPlayerExitZoneCompleteView<'_> {}

// SAFETY:
// - `PayloadPlayerExitZoneCompleteView` is `Send` because while its alive a `PayloadPlayerExitZoneCompleteMut` cannot.
// - `PayloadPlayerExitZoneCompleteView` does not use thread-local data.
unsafe impl Send for PayloadPlayerExitZoneCompleteView<'_> {}

impl<'msg> ::protobuf::Proxy<'msg> for PayloadPlayerExitZoneCompleteView<'msg> {}
impl<'msg> ::protobuf::ViewProxy<'msg> for PayloadPlayerExitZoneCompleteView<'msg> {}

impl<'msg> ::protobuf::AsView for PayloadPlayerExitZoneCompleteView<'msg> {
  type Proxied = PayloadPlayerExitZoneComplete;
  fn as_view(&self) -> ::protobuf::View<'msg, PayloadPlayerExitZoneComplete> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for PayloadPlayerExitZoneCompleteView<'msg> {
  fn into_view<'shorter>(self) -> PayloadPlayerExitZoneCompleteView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<PayloadPlayerExitZoneComplete> for PayloadPlayerExitZoneCompleteView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> PayloadPlayerExitZoneComplete {
    let mut dst = PayloadPlayerExitZoneComplete::new();
    let dst_raw = ::protobuf::__internal::runtime::UpbGetMessagePtrMut::get_raw_message_mut(&mut dst, ::protobuf::__internal::Private);
    let dst_arena = ::protobuf::__internal::runtime::UpbGetArena::get_arena(&mut dst, ::protobuf::__internal::Private);
    let src_raw = ::protobuf::__internal::runtime::UpbGetMessagePtr::get_raw_message(&self, ::protobuf::__internal::Private);

    unsafe { ::protobuf::__internal::runtime::upb_Message_DeepCopy(
      dst_raw,
      src_raw,
      <Self as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
      dst_arena.raw(),
    ) };
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<PayloadPlayerExitZoneComplete> for PayloadPlayerExitZoneCompleteMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> PayloadPlayerExitZoneComplete {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::runtime::UpbTypeConversions for PayloadPlayerExitZoneComplete {
    fn upb_type() -> ::protobuf::__internal::runtime::CType {
        ::protobuf::__internal::runtime::CType::Message
    }

    fn to_message_value(
        val: ::protobuf::View<'_, Self>) -> ::protobuf::__internal::runtime::upb_MessageValue {
        ::protobuf::__internal::runtime::upb_MessageValue { msg_val: Some(val.raw_msg()) }
    }

    unsafe fn into_message_value_fuse_if_required(
      raw_parent_arena: ::protobuf::__internal::runtime::RawArena,
      mut val: Self) -> ::protobuf::__internal::runtime::upb_MessageValue {
      // SAFETY: The arena memory is not freed due to `ManuallyDrop`.
      let parent_arena = ::std::mem::ManuallyDrop::new(
          unsafe { ::protobuf::__internal::runtime::Arena::from_raw(raw_parent_arena) });

      parent_arena.fuse(val.as_message_mut_inner(::protobuf::__internal::Private).arena());
      ::protobuf::__internal::runtime::upb_MessageValue { msg_val: Some(val.raw_msg()) }
    }

    unsafe fn from_message_value<'msg>(msg: ::protobuf::__internal::runtime::upb_MessageValue)
        -> ::protobuf::View<'msg, Self> {
        let raw = unsafe { msg.msg_val }.expect("expected present message value in map");
        let inner = unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap_raw(raw) };
        PayloadPlayerExitZoneCompleteView::new(::protobuf::__internal::Private, inner)
    }

    unsafe fn from_message_mut<'msg>(msg: ::protobuf::__internal::runtime::RawMessage, arena: &'msg ::protobuf::__internal::runtime::Arena)
        -> PayloadPlayerExitZoneCompleteMut<'msg> {
        let inner = unsafe { ::protobuf::__internal::runtime::MessageMutInner::<'msg, PayloadPlayerExitZoneComplete>::wrap_raw(msg, arena) };
        PayloadPlayerExitZoneCompleteMut::new(::protobuf::__internal::Private, inner)
    }
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct PayloadPlayerExitZoneCompleteMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, PayloadPlayerExitZoneComplete>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for PayloadPlayerExitZoneCompleteMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for PayloadPlayerExitZoneCompleteMut<'msg> {
  type Message = PayloadPlayerExitZoneComplete;
}

impl ::std::fmt::Debug for PayloadPlayerExitZoneCompleteMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    let string = unsafe {
      ::protobuf::__internal::runtime::debug_string(
        self.raw_msg(),
        <Self as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table()
      )
    };
    write!(f, "{}", string)
  }
}

impl ::protobuf::Serialize for PayloadPlayerExitZoneCompleteMut<'_> {
  fn serialize(&self) -> ::std::result::Result<Vec<u8>, ::protobuf::SerializeError> {
    ::protobuf::AsView::as_view(self).serialize()
  }
}

#[allow(dead_code)]
impl<'msg> PayloadPlayerExitZoneCompleteMut<'msg> {
  #[doc(hidden)]
  pub fn from_parent<ParentT: ::protobuf::Message>(
             _private: ::protobuf::__internal::Private,
             parent: ::protobuf::__internal::runtime::MessageMutInner<'msg, ParentT>,
             msg: ::protobuf::__internal::runtime::RawMessage)
    -> Self {
    Self {
      inner: ::protobuf::__internal::runtime::MessageMutInner::from_parent(parent, msg)
    }
  }

  #[doc(hidden)]
  pub fn new(_private: ::protobuf::__internal::Private, inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, PayloadPlayerExitZoneComplete>) -> Self {
    Self { inner }
  }

  fn raw_msg(&self) -> ::protobuf::__internal::runtime::RawMessage {
    self.inner.raw()
  }

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, PayloadPlayerExitZoneComplete> {
    self.inner
  }

  pub fn to_owned(&self) -> PayloadPlayerExitZoneComplete {
    ::protobuf::AsView::as_view(self).to_owned()
  }

  fn arena(&mut self) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }

  // user_id: optional uint64
  pub fn user_id(&self) -> u64 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_u64_at_index(
        0, (0u64).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_user_id(&mut self, val: u64) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_u64_at_index(
        0, val.into()
      )
    }
  }

}

// SAFETY:
// - `PayloadPlayerExitZoneCompleteMut` does not perform any shared mutation.
// - `PayloadPlayerExitZoneCompleteMut` is not `Send`, and so even in the presence of mutator
//   splitting, synchronous access of an arena is impossible.
unsafe impl Sync for PayloadPlayerExitZoneCompleteMut<'_> {}

impl<'msg> ::protobuf::Proxy<'msg> for PayloadPlayerExitZoneCompleteMut<'msg> {}
impl<'msg> ::protobuf::MutProxy<'msg> for PayloadPlayerExitZoneCompleteMut<'msg> {}

impl<'msg> ::protobuf::AsView for PayloadPlayerExitZoneCompleteMut<'msg> {
  type Proxied = PayloadPlayerExitZoneComplete;
  fn as_view(&self) -> ::protobuf::View<'_, PayloadPlayerExitZoneComplete> {
    PayloadPlayerExitZoneCompleteView {
      inner: ::protobuf::__internal::runtime::MessageViewInner::view_of_mut(self.inner.clone()),
      _phantom: ::std::marker::PhantomData
    }
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for PayloadPlayerExitZoneCompleteMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, PayloadPlayerExitZoneComplete>
  where
      'msg: 'shorter {
    PayloadPlayerExitZoneCompleteView {
      inner: ::protobuf::__internal::runtime::MessageViewInner::view_of_mut(self.inner.clone()),
      _phantom: ::std::marker::PhantomData
    }
  }
}

impl<'msg> ::protobuf::AsMut for PayloadPlayerExitZoneCompleteMut<'msg> {
  type MutProxied = PayloadPlayerExitZoneComplete;
  fn as_mut(&mut self) -> PayloadPlayerExitZoneCompleteMut<'msg> {
    PayloadPlayerExitZoneCompleteMut { inner: self.inner }
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for PayloadPlayerExitZoneCompleteMut<'msg> {
  fn into_mut<'shorter>(self) -> PayloadPlayerExitZoneCompleteMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl PayloadPlayerExitZoneComplete {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }

  fn raw_msg(&self) -> ::protobuf::__internal::runtime::RawMessage {
    self.inner.raw()
  }

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, PayloadPlayerExitZoneComplete> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
  }

  fn arena(&mut self) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }

  pub fn parse(data: &[u8]) -> ::std::result::Result<Self, ::protobuf::ParseError> {
    let mut msg = Self::new();
    ::protobuf::ClearAndParse::clear_and_parse(&mut msg, data).map(|_| msg)
  }

  pub fn parse_dont_enforce_required(data: &[u8]) -> ::std::result::Result<Self, ::protobuf::ParseError> {
    let mut msg = Self::new();
    ::protobuf::ClearAndParse::clear_and_parse_dont_enforce_required(&mut msg, data).map(|_| msg)
  }

  pub fn as_view(&self) -> PayloadPlayerExitZoneCompleteView {
    PayloadPlayerExitZoneCompleteView::new(
        ::protobuf::__internal::Private,
        ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner))
  }

  pub fn as_mut(&mut self) -> PayloadPlayerExitZoneCompleteMut {
    let inner = ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner);
    PayloadPlayerExitZoneCompleteMut::new(::protobuf::__internal::Private, inner)
  }

  // user_id: optional uint64
  pub fn user_id(&self) -> u64 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_u64_at_index(
        0, (0u64).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_user_id(&mut self, val: u64) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_u64_at_index(
        0, val.into()
      )
    }
  }

}  // impl PayloadPlayerExitZoneComplete

impl ::std::ops::Drop for PayloadPlayerExitZoneComplete {
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for PayloadPlayerExitZoneComplete {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for PayloadPlayerExitZoneComplete {
  type Proxied = Self;
  fn as_view(&self) -> PayloadPlayerExitZoneCompleteView {
    self.as_view()
  }
}

impl ::protobuf::AsMut for PayloadPlayerExitZoneComplete {
  type MutProxied = Self;
  fn as_mut(&mut self) -> PayloadPlayerExitZoneCompleteMut {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for PayloadPlayerExitZoneComplete {
  fn mini_table() -> *const ::protobuf::__internal::runtime::upb_MiniTable {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTablePtr> =
        ::std::sync::OnceLock::new();
    ONCE_LOCK.get_or_init(|| unsafe {
      super::Proto__PayloadPlayerExitZoneComplete_msg_init.0 =
          ::protobuf::__internal::runtime::upb_MiniTable_Build(
              "$,P".as_ptr(),
              3,
              ::protobuf::__internal::runtime::THREAD_LOCAL_ARENA.with(|a| a.raw()),
              ::std::ptr::null_mut());
      let submessages = [
      ];
      let subenums = [
      ];
      assert!(::protobuf::__internal::runtime::upb_MiniTable_Link(
          super::Proto__PayloadPlayerExitZoneComplete_msg_init.0,
          submessages.as_ptr() as *const *const ::protobuf::__internal::runtime::upb_MiniTable,
          submessages.len(), subenums.as_ptr(), subenums.len()));
      ::protobuf::__internal::runtime::MiniTablePtr(super::Proto__PayloadPlayerExitZoneComplete_msg_init.0)
    }).0
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for PayloadPlayerExitZoneComplete {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for PayloadPlayerExitZoneCompleteView<'_> {
  #[inline(always)]
  fn mini_table() -> *const ::protobuf::__internal::runtime::upb_MiniTable {
    <PayloadPlayerExitZoneComplete as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for PayloadPlayerExitZoneCompleteMut<'_> {
  #[inline(always)]
  fn mini_table() -> *const ::protobuf::__internal::runtime::upb_MiniTable {
    <PayloadPlayerExitZoneComplete as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for PayloadPlayerExitZoneComplete {
  type Msg = PayloadPlayerExitZoneComplete;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<PayloadPlayerExitZoneComplete> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for PayloadPlayerExitZoneComplete {
  type Msg = PayloadPlayerExitZoneComplete;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<PayloadPlayerExitZoneComplete> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for PayloadPlayerExitZoneCompleteMut<'_> {
  type Msg = PayloadPlayerExitZoneComplete;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<PayloadPlayerExitZoneComplete> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for PayloadPlayerExitZoneCompleteMut<'_> {
  type Msg = PayloadPlayerExitZoneComplete;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<PayloadPlayerExitZoneComplete> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for PayloadPlayerExitZoneCompleteView<'_> {
  type Msg = PayloadPlayerExitZoneComplete;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<PayloadPlayerExitZoneComplete> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for PayloadPlayerExitZoneCompleteMut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}


// upb kernel doesn't support any owned message or message mut interop.
impl ::protobuf::OwnedMessageInterop for PayloadPlayerExitZoneComplete {}
impl<'a> ::protobuf::MessageMutInterop<'a> for PayloadPlayerExitZoneCompleteMut<'a> {}

impl<'a> ::protobuf::MessageViewInterop<'a> for PayloadPlayerExitZoneCompleteView<'a> {
  unsafe fn __unstable_wrap_raw_message(
    msg: &'a *const ::std::ffi::c_void) -> Self {
    let raw = ::protobuf::__internal::runtime::RawMessage::new(*msg as *mut _).unwrap();
    let inner = unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap_raw(raw) };
    Self::new(::protobuf::__internal::Private, inner)
  }
  unsafe fn __unstable_wrap_raw_message_unchecked_lifetime(
    msg: *const ::std::ffi::c_void) -> Self {
    let raw = ::protobuf::__internal::runtime::RawMessage::new(msg as *mut _).unwrap();
    let inner = unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap_raw(raw) };
    Self::new(::protobuf::__internal::Private, inner)
  }
  fn __unstable_as_raw_message(&self) -> *const ::std::ffi::c_void {
    self.inner.raw().as_ptr() as *const _
  }
}

