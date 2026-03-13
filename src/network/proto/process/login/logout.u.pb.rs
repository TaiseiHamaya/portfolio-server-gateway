const _: () = ::protobuf::__internal::assert_compatible_gencode_version("4.32.1-release");
// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut Proto__PayloadLogoutRequest_msg_init: ::protobuf::__internal::runtime::MiniTablePtr =
    ::protobuf::__internal::runtime::MiniTablePtr(::std::ptr::null_mut());
#[allow(non_camel_case_types)]
pub struct PayloadLogoutRequest {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<PayloadLogoutRequest>
}

impl ::protobuf::Message for PayloadLogoutRequest {}

impl ::std::default::Default for PayloadLogoutRequest {
  fn default() -> Self {
    Self::new()
  }
}

impl ::protobuf::Parse for PayloadLogoutRequest {
  fn parse(serialized: &[u8]) -> ::std::result::Result<Self, ::protobuf::ParseError> {
    Self::parse(serialized)
  }

  fn parse_dont_enforce_required(serialized: &[u8]) -> ::std::result::Result<Self, ::protobuf::ParseError> {
    Self::parse_dont_enforce_required(serialized)
  }
}

impl ::std::fmt::Debug for PayloadLogoutRequest {
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

impl ::protobuf::Serialize for PayloadLogoutRequest {
  fn serialize(&self) -> ::std::result::Result<Vec<u8>, ::protobuf::SerializeError> {
    ::protobuf::AsView::as_view(self).serialize()
  }
}

// SAFETY:
// - `PayloadLogoutRequest` is `Sync` because it does not implement interior mutability.
//    Neither does `PayloadLogoutRequestMut`.
unsafe impl Sync for PayloadLogoutRequest {}

// SAFETY:
// - `PayloadLogoutRequest` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl Send for PayloadLogoutRequest {}

impl ::protobuf::Proxied for PayloadLogoutRequest {
  type View<'msg> = PayloadLogoutRequestView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for PayloadLogoutRequest {}

impl ::protobuf::MutProxied for PayloadLogoutRequest {
  type Mut<'msg> = PayloadLogoutRequestMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct PayloadLogoutRequestView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, PayloadLogoutRequest>,
  _phantom: ::std::marker::PhantomData<&'msg ()>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for PayloadLogoutRequestView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for PayloadLogoutRequestView<'msg> {
  type Message = PayloadLogoutRequest;
}

impl ::std::fmt::Debug for PayloadLogoutRequestView<'_> {
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

impl ::protobuf::Serialize for PayloadLogoutRequestView<'_> {
  fn serialize(&self) -> ::std::result::Result<Vec<u8>, ::protobuf::SerializeError> {
    // SAFETY: `MINI_TABLE` is the one associated with `self.raw_msg()`.
    let encoded = unsafe {
      ::protobuf::__internal::runtime::wire::encode(self.raw_msg(),
          <Self as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table())
    };
    encoded.map_err(|_| ::protobuf::SerializeError)
  }
}

impl ::std::default::Default for PayloadLogoutRequestView<'_> {
  fn default() -> PayloadLogoutRequestView<'static> {
    let inner = unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap_raw(::protobuf::__internal::runtime::ScratchSpace::zeroed_block()) };
    PayloadLogoutRequestView::new(::protobuf::__internal::Private, inner)
  }
}

#[allow(dead_code)]
impl<'msg> PayloadLogoutRequestView<'msg> {
  #[doc(hidden)]
  pub fn new(_private: ::protobuf::__internal::Private, inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, PayloadLogoutRequest>) -> Self {
    Self { inner, _phantom: ::std::marker::PhantomData }
  }

  fn raw_msg(&self) -> ::protobuf::__internal::runtime::RawMessage {
    self.inner.raw()
  }

  pub fn to_owned(&self) -> PayloadLogoutRequest {
    ::protobuf::IntoProxied::into_proxied(*self, ::protobuf::__internal::Private)
  }

  // id: optional uint64
  pub fn id(self) -> u64 {
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
// - `PayloadLogoutRequestView` is `Sync` because it does not support mutation.
unsafe impl Sync for PayloadLogoutRequestView<'_> {}

// SAFETY:
// - `PayloadLogoutRequestView` is `Send` because while its alive a `PayloadLogoutRequestMut` cannot.
// - `PayloadLogoutRequestView` does not use thread-local data.
unsafe impl Send for PayloadLogoutRequestView<'_> {}

impl<'msg> ::protobuf::Proxy<'msg> for PayloadLogoutRequestView<'msg> {}
impl<'msg> ::protobuf::ViewProxy<'msg> for PayloadLogoutRequestView<'msg> {}

impl<'msg> ::protobuf::AsView for PayloadLogoutRequestView<'msg> {
  type Proxied = PayloadLogoutRequest;
  fn as_view(&self) -> ::protobuf::View<'msg, PayloadLogoutRequest> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for PayloadLogoutRequestView<'msg> {
  fn into_view<'shorter>(self) -> PayloadLogoutRequestView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<PayloadLogoutRequest> for PayloadLogoutRequestView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> PayloadLogoutRequest {
    let mut dst = PayloadLogoutRequest::new();
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

impl<'msg> ::protobuf::IntoProxied<PayloadLogoutRequest> for PayloadLogoutRequestMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> PayloadLogoutRequest {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::runtime::UpbTypeConversions for PayloadLogoutRequest {
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
        PayloadLogoutRequestView::new(::protobuf::__internal::Private, inner)
    }

    unsafe fn from_message_mut<'msg>(msg: ::protobuf::__internal::runtime::RawMessage, arena: &'msg ::protobuf::__internal::runtime::Arena)
        -> PayloadLogoutRequestMut<'msg> {
        let inner = unsafe { ::protobuf::__internal::runtime::MessageMutInner::<'msg, PayloadLogoutRequest>::wrap_raw(msg, arena) };
        PayloadLogoutRequestMut::new(::protobuf::__internal::Private, inner)
    }
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct PayloadLogoutRequestMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, PayloadLogoutRequest>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for PayloadLogoutRequestMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for PayloadLogoutRequestMut<'msg> {
  type Message = PayloadLogoutRequest;
}

impl ::std::fmt::Debug for PayloadLogoutRequestMut<'_> {
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

impl ::protobuf::Serialize for PayloadLogoutRequestMut<'_> {
  fn serialize(&self) -> ::std::result::Result<Vec<u8>, ::protobuf::SerializeError> {
    ::protobuf::AsView::as_view(self).serialize()
  }
}

#[allow(dead_code)]
impl<'msg> PayloadLogoutRequestMut<'msg> {
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
  pub fn new(_private: ::protobuf::__internal::Private, inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, PayloadLogoutRequest>) -> Self {
    Self { inner }
  }

  fn raw_msg(&self) -> ::protobuf::__internal::runtime::RawMessage {
    self.inner.raw()
  }

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, PayloadLogoutRequest> {
    self.inner
  }

  pub fn to_owned(&self) -> PayloadLogoutRequest {
    ::protobuf::AsView::as_view(self).to_owned()
  }

  fn arena(&mut self) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }

  // id: optional uint64
  pub fn id(&self) -> u64 {
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
  pub fn set_id(&mut self, val: u64) {
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
// - `PayloadLogoutRequestMut` does not perform any shared mutation.
// - `PayloadLogoutRequestMut` is not `Send`, and so even in the presence of mutator
//   splitting, synchronous access of an arena is impossible.
unsafe impl Sync for PayloadLogoutRequestMut<'_> {}

impl<'msg> ::protobuf::Proxy<'msg> for PayloadLogoutRequestMut<'msg> {}
impl<'msg> ::protobuf::MutProxy<'msg> for PayloadLogoutRequestMut<'msg> {}

impl<'msg> ::protobuf::AsView for PayloadLogoutRequestMut<'msg> {
  type Proxied = PayloadLogoutRequest;
  fn as_view(&self) -> ::protobuf::View<'_, PayloadLogoutRequest> {
    PayloadLogoutRequestView {
      inner: ::protobuf::__internal::runtime::MessageViewInner::view_of_mut(self.inner.clone()),
      _phantom: ::std::marker::PhantomData
    }
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for PayloadLogoutRequestMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, PayloadLogoutRequest>
  where
      'msg: 'shorter {
    PayloadLogoutRequestView {
      inner: ::protobuf::__internal::runtime::MessageViewInner::view_of_mut(self.inner.clone()),
      _phantom: ::std::marker::PhantomData
    }
  }
}

impl<'msg> ::protobuf::AsMut for PayloadLogoutRequestMut<'msg> {
  type MutProxied = PayloadLogoutRequest;
  fn as_mut(&mut self) -> PayloadLogoutRequestMut<'msg> {
    PayloadLogoutRequestMut { inner: self.inner }
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for PayloadLogoutRequestMut<'msg> {
  fn into_mut<'shorter>(self) -> PayloadLogoutRequestMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl PayloadLogoutRequest {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }

  fn raw_msg(&self) -> ::protobuf::__internal::runtime::RawMessage {
    self.inner.raw()
  }

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, PayloadLogoutRequest> {
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

  pub fn as_view(&self) -> PayloadLogoutRequestView {
    PayloadLogoutRequestView::new(
        ::protobuf::__internal::Private,
        ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner))
  }

  pub fn as_mut(&mut self) -> PayloadLogoutRequestMut {
    let inner = ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner);
    PayloadLogoutRequestMut::new(::protobuf::__internal::Private, inner)
  }

  // id: optional uint64
  pub fn id(&self) -> u64 {
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
  pub fn set_id(&mut self, val: u64) {
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

}  // impl PayloadLogoutRequest

impl ::std::ops::Drop for PayloadLogoutRequest {
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for PayloadLogoutRequest {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for PayloadLogoutRequest {
  type Proxied = Self;
  fn as_view(&self) -> PayloadLogoutRequestView {
    self.as_view()
  }
}

impl ::protobuf::AsMut for PayloadLogoutRequest {
  type MutProxied = Self;
  fn as_mut(&mut self) -> PayloadLogoutRequestMut {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for PayloadLogoutRequest {
  fn mini_table() -> *const ::protobuf::__internal::runtime::upb_MiniTable {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTablePtr> =
        ::std::sync::OnceLock::new();
    ONCE_LOCK.get_or_init(|| unsafe {
      super::Proto__PayloadLogoutRequest_msg_init.0 =
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
          super::Proto__PayloadLogoutRequest_msg_init.0,
          submessages.as_ptr() as *const *const ::protobuf::__internal::runtime::upb_MiniTable,
          submessages.len(), subenums.as_ptr(), subenums.len()));
      ::protobuf::__internal::runtime::MiniTablePtr(super::Proto__PayloadLogoutRequest_msg_init.0)
    }).0
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for PayloadLogoutRequest {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for PayloadLogoutRequestView<'_> {
  #[inline(always)]
  fn mini_table() -> *const ::protobuf::__internal::runtime::upb_MiniTable {
    <PayloadLogoutRequest as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for PayloadLogoutRequestMut<'_> {
  #[inline(always)]
  fn mini_table() -> *const ::protobuf::__internal::runtime::upb_MiniTable {
    <PayloadLogoutRequest as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for PayloadLogoutRequest {
  type Msg = PayloadLogoutRequest;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<PayloadLogoutRequest> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for PayloadLogoutRequest {
  type Msg = PayloadLogoutRequest;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<PayloadLogoutRequest> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for PayloadLogoutRequestMut<'_> {
  type Msg = PayloadLogoutRequest;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<PayloadLogoutRequest> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for PayloadLogoutRequestMut<'_> {
  type Msg = PayloadLogoutRequest;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<PayloadLogoutRequest> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for PayloadLogoutRequestView<'_> {
  type Msg = PayloadLogoutRequest;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<PayloadLogoutRequest> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for PayloadLogoutRequestMut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}


// upb kernel doesn't support any owned message or message mut interop.
impl ::protobuf::OwnedMessageInterop for PayloadLogoutRequest {}
impl<'a> ::protobuf::MessageMutInterop<'a> for PayloadLogoutRequestMut<'a> {}

impl<'a> ::protobuf::MessageViewInterop<'a> for PayloadLogoutRequestView<'a> {
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
pub(crate) static mut Proto__PayloadLogoutResponse_msg_init: ::protobuf::__internal::runtime::MiniTablePtr =
    ::protobuf::__internal::runtime::MiniTablePtr(::std::ptr::null_mut());
#[allow(non_camel_case_types)]
pub struct PayloadLogoutResponse {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<PayloadLogoutResponse>
}

impl ::protobuf::Message for PayloadLogoutResponse {}

impl ::std::default::Default for PayloadLogoutResponse {
  fn default() -> Self {
    Self::new()
  }
}

impl ::protobuf::Parse for PayloadLogoutResponse {
  fn parse(serialized: &[u8]) -> ::std::result::Result<Self, ::protobuf::ParseError> {
    Self::parse(serialized)
  }

  fn parse_dont_enforce_required(serialized: &[u8]) -> ::std::result::Result<Self, ::protobuf::ParseError> {
    Self::parse_dont_enforce_required(serialized)
  }
}

impl ::std::fmt::Debug for PayloadLogoutResponse {
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

impl ::protobuf::Serialize for PayloadLogoutResponse {
  fn serialize(&self) -> ::std::result::Result<Vec<u8>, ::protobuf::SerializeError> {
    ::protobuf::AsView::as_view(self).serialize()
  }
}

// SAFETY:
// - `PayloadLogoutResponse` is `Sync` because it does not implement interior mutability.
//    Neither does `PayloadLogoutResponseMut`.
unsafe impl Sync for PayloadLogoutResponse {}

// SAFETY:
// - `PayloadLogoutResponse` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl Send for PayloadLogoutResponse {}

impl ::protobuf::Proxied for PayloadLogoutResponse {
  type View<'msg> = PayloadLogoutResponseView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for PayloadLogoutResponse {}

impl ::protobuf::MutProxied for PayloadLogoutResponse {
  type Mut<'msg> = PayloadLogoutResponseMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct PayloadLogoutResponseView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, PayloadLogoutResponse>,
  _phantom: ::std::marker::PhantomData<&'msg ()>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for PayloadLogoutResponseView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for PayloadLogoutResponseView<'msg> {
  type Message = PayloadLogoutResponse;
}

impl ::std::fmt::Debug for PayloadLogoutResponseView<'_> {
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

impl ::protobuf::Serialize for PayloadLogoutResponseView<'_> {
  fn serialize(&self) -> ::std::result::Result<Vec<u8>, ::protobuf::SerializeError> {
    // SAFETY: `MINI_TABLE` is the one associated with `self.raw_msg()`.
    let encoded = unsafe {
      ::protobuf::__internal::runtime::wire::encode(self.raw_msg(),
          <Self as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table())
    };
    encoded.map_err(|_| ::protobuf::SerializeError)
  }
}

impl ::std::default::Default for PayloadLogoutResponseView<'_> {
  fn default() -> PayloadLogoutResponseView<'static> {
    let inner = unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap_raw(::protobuf::__internal::runtime::ScratchSpace::zeroed_block()) };
    PayloadLogoutResponseView::new(::protobuf::__internal::Private, inner)
  }
}

#[allow(dead_code)]
impl<'msg> PayloadLogoutResponseView<'msg> {
  #[doc(hidden)]
  pub fn new(_private: ::protobuf::__internal::Private, inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, PayloadLogoutResponse>) -> Self {
    Self { inner, _phantom: ::std::marker::PhantomData }
  }

  fn raw_msg(&self) -> ::protobuf::__internal::runtime::RawMessage {
    self.inner.raw()
  }

  pub fn to_owned(&self) -> PayloadLogoutResponse {
    ::protobuf::IntoProxied::into_proxied(*self, ::protobuf::__internal::Private)
  }

  // is_succeeded: optional bool
  pub fn is_succeeded(self) -> bool {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_bool_at_index(
        0, (false).into()
      ).try_into().unwrap()
    }
  }

}

// SAFETY:
// - `PayloadLogoutResponseView` is `Sync` because it does not support mutation.
unsafe impl Sync for PayloadLogoutResponseView<'_> {}

// SAFETY:
// - `PayloadLogoutResponseView` is `Send` because while its alive a `PayloadLogoutResponseMut` cannot.
// - `PayloadLogoutResponseView` does not use thread-local data.
unsafe impl Send for PayloadLogoutResponseView<'_> {}

impl<'msg> ::protobuf::Proxy<'msg> for PayloadLogoutResponseView<'msg> {}
impl<'msg> ::protobuf::ViewProxy<'msg> for PayloadLogoutResponseView<'msg> {}

impl<'msg> ::protobuf::AsView for PayloadLogoutResponseView<'msg> {
  type Proxied = PayloadLogoutResponse;
  fn as_view(&self) -> ::protobuf::View<'msg, PayloadLogoutResponse> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for PayloadLogoutResponseView<'msg> {
  fn into_view<'shorter>(self) -> PayloadLogoutResponseView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<PayloadLogoutResponse> for PayloadLogoutResponseView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> PayloadLogoutResponse {
    let mut dst = PayloadLogoutResponse::new();
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

impl<'msg> ::protobuf::IntoProxied<PayloadLogoutResponse> for PayloadLogoutResponseMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> PayloadLogoutResponse {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::runtime::UpbTypeConversions for PayloadLogoutResponse {
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
        PayloadLogoutResponseView::new(::protobuf::__internal::Private, inner)
    }

    unsafe fn from_message_mut<'msg>(msg: ::protobuf::__internal::runtime::RawMessage, arena: &'msg ::protobuf::__internal::runtime::Arena)
        -> PayloadLogoutResponseMut<'msg> {
        let inner = unsafe { ::protobuf::__internal::runtime::MessageMutInner::<'msg, PayloadLogoutResponse>::wrap_raw(msg, arena) };
        PayloadLogoutResponseMut::new(::protobuf::__internal::Private, inner)
    }
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct PayloadLogoutResponseMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, PayloadLogoutResponse>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for PayloadLogoutResponseMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for PayloadLogoutResponseMut<'msg> {
  type Message = PayloadLogoutResponse;
}

impl ::std::fmt::Debug for PayloadLogoutResponseMut<'_> {
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

impl ::protobuf::Serialize for PayloadLogoutResponseMut<'_> {
  fn serialize(&self) -> ::std::result::Result<Vec<u8>, ::protobuf::SerializeError> {
    ::protobuf::AsView::as_view(self).serialize()
  }
}

#[allow(dead_code)]
impl<'msg> PayloadLogoutResponseMut<'msg> {
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
  pub fn new(_private: ::protobuf::__internal::Private, inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, PayloadLogoutResponse>) -> Self {
    Self { inner }
  }

  fn raw_msg(&self) -> ::protobuf::__internal::runtime::RawMessage {
    self.inner.raw()
  }

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, PayloadLogoutResponse> {
    self.inner
  }

  pub fn to_owned(&self) -> PayloadLogoutResponse {
    ::protobuf::AsView::as_view(self).to_owned()
  }

  fn arena(&mut self) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }

  // is_succeeded: optional bool
  pub fn is_succeeded(&self) -> bool {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_bool_at_index(
        0, (false).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_is_succeeded(&mut self, val: bool) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_bool_at_index(
        0, val.into()
      )
    }
  }

}

// SAFETY:
// - `PayloadLogoutResponseMut` does not perform any shared mutation.
// - `PayloadLogoutResponseMut` is not `Send`, and so even in the presence of mutator
//   splitting, synchronous access of an arena is impossible.
unsafe impl Sync for PayloadLogoutResponseMut<'_> {}

impl<'msg> ::protobuf::Proxy<'msg> for PayloadLogoutResponseMut<'msg> {}
impl<'msg> ::protobuf::MutProxy<'msg> for PayloadLogoutResponseMut<'msg> {}

impl<'msg> ::protobuf::AsView for PayloadLogoutResponseMut<'msg> {
  type Proxied = PayloadLogoutResponse;
  fn as_view(&self) -> ::protobuf::View<'_, PayloadLogoutResponse> {
    PayloadLogoutResponseView {
      inner: ::protobuf::__internal::runtime::MessageViewInner::view_of_mut(self.inner.clone()),
      _phantom: ::std::marker::PhantomData
    }
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for PayloadLogoutResponseMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, PayloadLogoutResponse>
  where
      'msg: 'shorter {
    PayloadLogoutResponseView {
      inner: ::protobuf::__internal::runtime::MessageViewInner::view_of_mut(self.inner.clone()),
      _phantom: ::std::marker::PhantomData
    }
  }
}

impl<'msg> ::protobuf::AsMut for PayloadLogoutResponseMut<'msg> {
  type MutProxied = PayloadLogoutResponse;
  fn as_mut(&mut self) -> PayloadLogoutResponseMut<'msg> {
    PayloadLogoutResponseMut { inner: self.inner }
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for PayloadLogoutResponseMut<'msg> {
  fn into_mut<'shorter>(self) -> PayloadLogoutResponseMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl PayloadLogoutResponse {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }

  fn raw_msg(&self) -> ::protobuf::__internal::runtime::RawMessage {
    self.inner.raw()
  }

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, PayloadLogoutResponse> {
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

  pub fn as_view(&self) -> PayloadLogoutResponseView {
    PayloadLogoutResponseView::new(
        ::protobuf::__internal::Private,
        ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner))
  }

  pub fn as_mut(&mut self) -> PayloadLogoutResponseMut {
    let inner = ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner);
    PayloadLogoutResponseMut::new(::protobuf::__internal::Private, inner)
  }

  // is_succeeded: optional bool
  pub fn is_succeeded(&self) -> bool {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_bool_at_index(
        0, (false).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_is_succeeded(&mut self, val: bool) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_bool_at_index(
        0, val.into()
      )
    }
  }

}  // impl PayloadLogoutResponse

impl ::std::ops::Drop for PayloadLogoutResponse {
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for PayloadLogoutResponse {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for PayloadLogoutResponse {
  type Proxied = Self;
  fn as_view(&self) -> PayloadLogoutResponseView {
    self.as_view()
  }
}

impl ::protobuf::AsMut for PayloadLogoutResponse {
  type MutProxied = Self;
  fn as_mut(&mut self) -> PayloadLogoutResponseMut {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for PayloadLogoutResponse {
  fn mini_table() -> *const ::protobuf::__internal::runtime::upb_MiniTable {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTablePtr> =
        ::std::sync::OnceLock::new();
    ONCE_LOCK.get_or_init(|| unsafe {
      super::Proto__PayloadLogoutResponse_msg_init.0 =
          ::protobuf::__internal::runtime::upb_MiniTable_Build(
              "$/P".as_ptr(),
              3,
              ::protobuf::__internal::runtime::THREAD_LOCAL_ARENA.with(|a| a.raw()),
              ::std::ptr::null_mut());
      let submessages = [
      ];
      let subenums = [
      ];
      assert!(::protobuf::__internal::runtime::upb_MiniTable_Link(
          super::Proto__PayloadLogoutResponse_msg_init.0,
          submessages.as_ptr() as *const *const ::protobuf::__internal::runtime::upb_MiniTable,
          submessages.len(), subenums.as_ptr(), subenums.len()));
      ::protobuf::__internal::runtime::MiniTablePtr(super::Proto__PayloadLogoutResponse_msg_init.0)
    }).0
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for PayloadLogoutResponse {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for PayloadLogoutResponseView<'_> {
  #[inline(always)]
  fn mini_table() -> *const ::protobuf::__internal::runtime::upb_MiniTable {
    <PayloadLogoutResponse as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for PayloadLogoutResponseMut<'_> {
  #[inline(always)]
  fn mini_table() -> *const ::protobuf::__internal::runtime::upb_MiniTable {
    <PayloadLogoutResponse as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for PayloadLogoutResponse {
  type Msg = PayloadLogoutResponse;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<PayloadLogoutResponse> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for PayloadLogoutResponse {
  type Msg = PayloadLogoutResponse;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<PayloadLogoutResponse> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for PayloadLogoutResponseMut<'_> {
  type Msg = PayloadLogoutResponse;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<PayloadLogoutResponse> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for PayloadLogoutResponseMut<'_> {
  type Msg = PayloadLogoutResponse;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<PayloadLogoutResponse> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for PayloadLogoutResponseView<'_> {
  type Msg = PayloadLogoutResponse;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<PayloadLogoutResponse> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for PayloadLogoutResponseMut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}


// upb kernel doesn't support any owned message or message mut interop.
impl ::protobuf::OwnedMessageInterop for PayloadLogoutResponse {}
impl<'a> ::protobuf::MessageMutInterop<'a> for PayloadLogoutResponseMut<'a> {}

impl<'a> ::protobuf::MessageViewInterop<'a> for PayloadLogoutResponseView<'a> {
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
pub(crate) static mut Proto__PayloadLogoutNotification_msg_init: ::protobuf::__internal::runtime::MiniTablePtr =
    ::protobuf::__internal::runtime::MiniTablePtr(::std::ptr::null_mut());
#[allow(non_camel_case_types)]
pub struct PayloadLogoutNotification {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<PayloadLogoutNotification>
}

impl ::protobuf::Message for PayloadLogoutNotification {}

impl ::std::default::Default for PayloadLogoutNotification {
  fn default() -> Self {
    Self::new()
  }
}

impl ::protobuf::Parse for PayloadLogoutNotification {
  fn parse(serialized: &[u8]) -> ::std::result::Result<Self, ::protobuf::ParseError> {
    Self::parse(serialized)
  }

  fn parse_dont_enforce_required(serialized: &[u8]) -> ::std::result::Result<Self, ::protobuf::ParseError> {
    Self::parse_dont_enforce_required(serialized)
  }
}

impl ::std::fmt::Debug for PayloadLogoutNotification {
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

impl ::protobuf::Serialize for PayloadLogoutNotification {
  fn serialize(&self) -> ::std::result::Result<Vec<u8>, ::protobuf::SerializeError> {
    ::protobuf::AsView::as_view(self).serialize()
  }
}

// SAFETY:
// - `PayloadLogoutNotification` is `Sync` because it does not implement interior mutability.
//    Neither does `PayloadLogoutNotificationMut`.
unsafe impl Sync for PayloadLogoutNotification {}

// SAFETY:
// - `PayloadLogoutNotification` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl Send for PayloadLogoutNotification {}

impl ::protobuf::Proxied for PayloadLogoutNotification {
  type View<'msg> = PayloadLogoutNotificationView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for PayloadLogoutNotification {}

impl ::protobuf::MutProxied for PayloadLogoutNotification {
  type Mut<'msg> = PayloadLogoutNotificationMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct PayloadLogoutNotificationView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, PayloadLogoutNotification>,
  _phantom: ::std::marker::PhantomData<&'msg ()>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for PayloadLogoutNotificationView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for PayloadLogoutNotificationView<'msg> {
  type Message = PayloadLogoutNotification;
}

impl ::std::fmt::Debug for PayloadLogoutNotificationView<'_> {
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

impl ::protobuf::Serialize for PayloadLogoutNotificationView<'_> {
  fn serialize(&self) -> ::std::result::Result<Vec<u8>, ::protobuf::SerializeError> {
    // SAFETY: `MINI_TABLE` is the one associated with `self.raw_msg()`.
    let encoded = unsafe {
      ::protobuf::__internal::runtime::wire::encode(self.raw_msg(),
          <Self as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table())
    };
    encoded.map_err(|_| ::protobuf::SerializeError)
  }
}

impl ::std::default::Default for PayloadLogoutNotificationView<'_> {
  fn default() -> PayloadLogoutNotificationView<'static> {
    let inner = unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap_raw(::protobuf::__internal::runtime::ScratchSpace::zeroed_block()) };
    PayloadLogoutNotificationView::new(::protobuf::__internal::Private, inner)
  }
}

#[allow(dead_code)]
impl<'msg> PayloadLogoutNotificationView<'msg> {
  #[doc(hidden)]
  pub fn new(_private: ::protobuf::__internal::Private, inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, PayloadLogoutNotification>) -> Self {
    Self { inner, _phantom: ::std::marker::PhantomData }
  }

  fn raw_msg(&self) -> ::protobuf::__internal::runtime::RawMessage {
    self.inner.raw()
  }

  pub fn to_owned(&self) -> PayloadLogoutNotification {
    ::protobuf::IntoProxied::into_proxied(*self, ::protobuf::__internal::Private)
  }

  // id: optional uint64
  pub fn id(self) -> u64 {
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
// - `PayloadLogoutNotificationView` is `Sync` because it does not support mutation.
unsafe impl Sync for PayloadLogoutNotificationView<'_> {}

// SAFETY:
// - `PayloadLogoutNotificationView` is `Send` because while its alive a `PayloadLogoutNotificationMut` cannot.
// - `PayloadLogoutNotificationView` does not use thread-local data.
unsafe impl Send for PayloadLogoutNotificationView<'_> {}

impl<'msg> ::protobuf::Proxy<'msg> for PayloadLogoutNotificationView<'msg> {}
impl<'msg> ::protobuf::ViewProxy<'msg> for PayloadLogoutNotificationView<'msg> {}

impl<'msg> ::protobuf::AsView for PayloadLogoutNotificationView<'msg> {
  type Proxied = PayloadLogoutNotification;
  fn as_view(&self) -> ::protobuf::View<'msg, PayloadLogoutNotification> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for PayloadLogoutNotificationView<'msg> {
  fn into_view<'shorter>(self) -> PayloadLogoutNotificationView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<PayloadLogoutNotification> for PayloadLogoutNotificationView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> PayloadLogoutNotification {
    let mut dst = PayloadLogoutNotification::new();
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

impl<'msg> ::protobuf::IntoProxied<PayloadLogoutNotification> for PayloadLogoutNotificationMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> PayloadLogoutNotification {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::runtime::UpbTypeConversions for PayloadLogoutNotification {
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
        PayloadLogoutNotificationView::new(::protobuf::__internal::Private, inner)
    }

    unsafe fn from_message_mut<'msg>(msg: ::protobuf::__internal::runtime::RawMessage, arena: &'msg ::protobuf::__internal::runtime::Arena)
        -> PayloadLogoutNotificationMut<'msg> {
        let inner = unsafe { ::protobuf::__internal::runtime::MessageMutInner::<'msg, PayloadLogoutNotification>::wrap_raw(msg, arena) };
        PayloadLogoutNotificationMut::new(::protobuf::__internal::Private, inner)
    }
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct PayloadLogoutNotificationMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, PayloadLogoutNotification>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for PayloadLogoutNotificationMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for PayloadLogoutNotificationMut<'msg> {
  type Message = PayloadLogoutNotification;
}

impl ::std::fmt::Debug for PayloadLogoutNotificationMut<'_> {
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

impl ::protobuf::Serialize for PayloadLogoutNotificationMut<'_> {
  fn serialize(&self) -> ::std::result::Result<Vec<u8>, ::protobuf::SerializeError> {
    ::protobuf::AsView::as_view(self).serialize()
  }
}

#[allow(dead_code)]
impl<'msg> PayloadLogoutNotificationMut<'msg> {
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
  pub fn new(_private: ::protobuf::__internal::Private, inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, PayloadLogoutNotification>) -> Self {
    Self { inner }
  }

  fn raw_msg(&self) -> ::protobuf::__internal::runtime::RawMessage {
    self.inner.raw()
  }

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, PayloadLogoutNotification> {
    self.inner
  }

  pub fn to_owned(&self) -> PayloadLogoutNotification {
    ::protobuf::AsView::as_view(self).to_owned()
  }

  fn arena(&mut self) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }

  // id: optional uint64
  pub fn id(&self) -> u64 {
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
  pub fn set_id(&mut self, val: u64) {
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
// - `PayloadLogoutNotificationMut` does not perform any shared mutation.
// - `PayloadLogoutNotificationMut` is not `Send`, and so even in the presence of mutator
//   splitting, synchronous access of an arena is impossible.
unsafe impl Sync for PayloadLogoutNotificationMut<'_> {}

impl<'msg> ::protobuf::Proxy<'msg> for PayloadLogoutNotificationMut<'msg> {}
impl<'msg> ::protobuf::MutProxy<'msg> for PayloadLogoutNotificationMut<'msg> {}

impl<'msg> ::protobuf::AsView for PayloadLogoutNotificationMut<'msg> {
  type Proxied = PayloadLogoutNotification;
  fn as_view(&self) -> ::protobuf::View<'_, PayloadLogoutNotification> {
    PayloadLogoutNotificationView {
      inner: ::protobuf::__internal::runtime::MessageViewInner::view_of_mut(self.inner.clone()),
      _phantom: ::std::marker::PhantomData
    }
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for PayloadLogoutNotificationMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, PayloadLogoutNotification>
  where
      'msg: 'shorter {
    PayloadLogoutNotificationView {
      inner: ::protobuf::__internal::runtime::MessageViewInner::view_of_mut(self.inner.clone()),
      _phantom: ::std::marker::PhantomData
    }
  }
}

impl<'msg> ::protobuf::AsMut for PayloadLogoutNotificationMut<'msg> {
  type MutProxied = PayloadLogoutNotification;
  fn as_mut(&mut self) -> PayloadLogoutNotificationMut<'msg> {
    PayloadLogoutNotificationMut { inner: self.inner }
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for PayloadLogoutNotificationMut<'msg> {
  fn into_mut<'shorter>(self) -> PayloadLogoutNotificationMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl PayloadLogoutNotification {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }

  fn raw_msg(&self) -> ::protobuf::__internal::runtime::RawMessage {
    self.inner.raw()
  }

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, PayloadLogoutNotification> {
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

  pub fn as_view(&self) -> PayloadLogoutNotificationView {
    PayloadLogoutNotificationView::new(
        ::protobuf::__internal::Private,
        ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner))
  }

  pub fn as_mut(&mut self) -> PayloadLogoutNotificationMut {
    let inner = ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner);
    PayloadLogoutNotificationMut::new(::protobuf::__internal::Private, inner)
  }

  // id: optional uint64
  pub fn id(&self) -> u64 {
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
  pub fn set_id(&mut self, val: u64) {
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

}  // impl PayloadLogoutNotification

impl ::std::ops::Drop for PayloadLogoutNotification {
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for PayloadLogoutNotification {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for PayloadLogoutNotification {
  type Proxied = Self;
  fn as_view(&self) -> PayloadLogoutNotificationView {
    self.as_view()
  }
}

impl ::protobuf::AsMut for PayloadLogoutNotification {
  type MutProxied = Self;
  fn as_mut(&mut self) -> PayloadLogoutNotificationMut {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for PayloadLogoutNotification {
  fn mini_table() -> *const ::protobuf::__internal::runtime::upb_MiniTable {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTablePtr> =
        ::std::sync::OnceLock::new();
    ONCE_LOCK.get_or_init(|| unsafe {
      super::Proto__PayloadLogoutNotification_msg_init.0 =
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
          super::Proto__PayloadLogoutNotification_msg_init.0,
          submessages.as_ptr() as *const *const ::protobuf::__internal::runtime::upb_MiniTable,
          submessages.len(), subenums.as_ptr(), subenums.len()));
      ::protobuf::__internal::runtime::MiniTablePtr(super::Proto__PayloadLogoutNotification_msg_init.0)
    }).0
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for PayloadLogoutNotification {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for PayloadLogoutNotificationView<'_> {
  #[inline(always)]
  fn mini_table() -> *const ::protobuf::__internal::runtime::upb_MiniTable {
    <PayloadLogoutNotification as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for PayloadLogoutNotificationMut<'_> {
  #[inline(always)]
  fn mini_table() -> *const ::protobuf::__internal::runtime::upb_MiniTable {
    <PayloadLogoutNotification as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for PayloadLogoutNotification {
  type Msg = PayloadLogoutNotification;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<PayloadLogoutNotification> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for PayloadLogoutNotification {
  type Msg = PayloadLogoutNotification;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<PayloadLogoutNotification> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for PayloadLogoutNotificationMut<'_> {
  type Msg = PayloadLogoutNotification;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<PayloadLogoutNotification> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for PayloadLogoutNotificationMut<'_> {
  type Msg = PayloadLogoutNotification;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<PayloadLogoutNotification> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for PayloadLogoutNotificationView<'_> {
  type Msg = PayloadLogoutNotification;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<PayloadLogoutNotification> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for PayloadLogoutNotificationMut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}


// upb kernel doesn't support any owned message or message mut interop.
impl ::protobuf::OwnedMessageInterop for PayloadLogoutNotification {}
impl<'a> ::protobuf::MessageMutInterop<'a> for PayloadLogoutNotificationMut<'a> {}

impl<'a> ::protobuf::MessageViewInterop<'a> for PayloadLogoutNotificationView<'a> {
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

#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct CategoryLogoutMessage(i32);

#[allow(non_upper_case_globals)]
impl CategoryLogoutMessage {
  pub const LogoutRequest: CategoryLogoutMessage = CategoryLogoutMessage(0);
  pub const LogoutResponse: CategoryLogoutMessage = CategoryLogoutMessage(1);
  pub const LogoutNotification: CategoryLogoutMessage = CategoryLogoutMessage(2);

  fn constant_name(&self) -> ::std::option::Option<&'static str> {
    #[allow(unreachable_patterns)] // In the case of aliases, just emit them all and let the first one match.
    Some(match self.0 {
      0 => "LogoutRequest",
      1 => "LogoutResponse",
      2 => "LogoutNotification",
      _ => return None
    })
  }
}

impl ::std::convert::From<CategoryLogoutMessage> for i32 {
  fn from(val: CategoryLogoutMessage) -> i32 {
    val.0
  }
}

impl ::std::convert::From<i32> for CategoryLogoutMessage {
  fn from(val: i32) -> CategoryLogoutMessage {
    Self(val)
  }
}

impl ::std::default::Default for CategoryLogoutMessage {
  fn default() -> Self {
    Self(0)
  }
}

impl ::std::fmt::Debug for CategoryLogoutMessage {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    if let Some(constant_name) = self.constant_name() {
      write!(f, "CategoryLogoutMessage::{}", constant_name)
    } else {
      write!(f, "CategoryLogoutMessage::from({})", self.0)
    }
  }
}

impl ::protobuf::IntoProxied<i32> for CategoryLogoutMessage {
  fn into_proxied(self, _: ::protobuf::__internal::Private) -> i32 {
    self.0
  }
}

impl ::protobuf::__internal::SealedInternal for CategoryLogoutMessage {}

impl ::protobuf::Proxied for CategoryLogoutMessage {
  type View<'a> = CategoryLogoutMessage;
}

impl ::protobuf::Proxy<'_> for CategoryLogoutMessage {}
impl ::protobuf::ViewProxy<'_> for CategoryLogoutMessage {}

impl ::protobuf::AsView for CategoryLogoutMessage {
  type Proxied = CategoryLogoutMessage;

  fn as_view(&self) -> CategoryLogoutMessage {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for CategoryLogoutMessage {
  fn into_view<'shorter>(self) -> CategoryLogoutMessage where 'msg: 'shorter {
    self
  }
}

unsafe impl ::protobuf::ProxiedInRepeated for CategoryLogoutMessage {
  fn repeated_new(_private: ::protobuf::__internal::Private) -> ::protobuf::Repeated<Self> {
    ::protobuf::__internal::runtime::new_enum_repeated()
  }

  unsafe fn repeated_free(_private: ::protobuf::__internal::Private, f: &mut ::protobuf::Repeated<Self>) {
    ::protobuf::__internal::runtime::free_enum_repeated(f)
  }

  fn repeated_len(r: ::protobuf::View<::protobuf::Repeated<Self>>) -> usize {
    ::protobuf::__internal::runtime::cast_enum_repeated_view(r).len()
  }

  fn repeated_push(r: ::protobuf::Mut<::protobuf::Repeated<Self>>, val: impl ::protobuf::IntoProxied<CategoryLogoutMessage>) {
    ::protobuf::__internal::runtime::cast_enum_repeated_mut(r).push(val.into_proxied(::protobuf::__internal::Private))
  }

  fn repeated_clear(r: ::protobuf::Mut<::protobuf::Repeated<Self>>) {
    ::protobuf::__internal::runtime::cast_enum_repeated_mut(r).clear()
  }

  unsafe fn repeated_get_unchecked(
      r: ::protobuf::View<::protobuf::Repeated<Self>>,
      index: usize,
  ) -> ::protobuf::View<CategoryLogoutMessage> {
    // SAFETY: In-bounds as promised by the caller.
    unsafe {
      ::protobuf::__internal::runtime::cast_enum_repeated_view(r)
        .get_unchecked(index)
        .try_into()
        .unwrap_unchecked()
    }
  }

  unsafe fn repeated_set_unchecked(
      r: ::protobuf::Mut<::protobuf::Repeated<Self>>,
      index: usize,
      val: impl ::protobuf::IntoProxied<CategoryLogoutMessage>,
  ) {
    // SAFETY: In-bounds as promised by the caller.
    unsafe {
      ::protobuf::__internal::runtime::cast_enum_repeated_mut(r)
        .set_unchecked(index, val.into_proxied(::protobuf::__internal::Private))
    }
  }

  fn repeated_copy_from(
      src: ::protobuf::View<::protobuf::Repeated<Self>>,
      dest: ::protobuf::Mut<::protobuf::Repeated<Self>>,
  ) {
    ::protobuf::__internal::runtime::cast_enum_repeated_mut(dest)
      .copy_from(::protobuf::__internal::runtime::cast_enum_repeated_view(src))
  }

  fn repeated_reserve(
      r: ::protobuf::Mut<::protobuf::Repeated<Self>>,
      additional: usize,
  ) {
      // SAFETY:
      // - `f.as_raw()` is valid.
      ::protobuf::__internal::runtime::reserve_enum_repeated_mut(r, additional);
  }
}

// SAFETY: this is an enum type
unsafe impl ::protobuf::__internal::Enum for CategoryLogoutMessage {
  const NAME: &'static str = "CategoryLogoutMessage";

  fn is_known(value: i32) -> bool {
    matches!(value, 0|1|2)
  }
}

impl ::protobuf::__internal::runtime::UpbTypeConversions for CategoryLogoutMessage {
    fn upb_type() -> ::protobuf::__internal::runtime::CType {
        ::protobuf::__internal::runtime::CType::Enum
    }

    fn to_message_value(
        val: ::protobuf::View<'_, Self>) -> ::protobuf::__internal::runtime::upb_MessageValue {
        ::protobuf::__internal::runtime::upb_MessageValue { int32_val: val.0 }
    }

    unsafe fn into_message_value_fuse_if_required(
      _raw_parent_arena: ::protobuf::__internal::runtime::RawArena,
      val: Self) -> ::protobuf::__internal::runtime::upb_MessageValue {
        ::protobuf::__internal::runtime::upb_MessageValue { int32_val: val.0 }
    }

    unsafe fn from_message_value<'msg>(val: ::protobuf::__internal::runtime::upb_MessageValue)
        -> ::protobuf::View<'msg, Self> {
      CategoryLogoutMessage(unsafe { val.int32_val })
    }
}


