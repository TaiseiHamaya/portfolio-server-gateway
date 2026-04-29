const _: () = ::protobuf::__internal::assert_compatible_gencode_version("4.32.1-release");
// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut Proto__PayloadLobbyStartGameRequest_msg_init: ::protobuf::__internal::runtime::MiniTablePtr =
    ::protobuf::__internal::runtime::MiniTablePtr(::std::ptr::null_mut());
#[allow(non_camel_case_types)]
pub struct PayloadLobbyStartGameRequest {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<PayloadLobbyStartGameRequest>
}

impl ::protobuf::Message for PayloadLobbyStartGameRequest {}

impl ::std::default::Default for PayloadLobbyStartGameRequest {
  fn default() -> Self {
    Self::new()
  }
}

impl ::protobuf::Parse for PayloadLobbyStartGameRequest {
  fn parse(serialized: &[u8]) -> ::std::result::Result<Self, ::protobuf::ParseError> {
    Self::parse(serialized)
  }

  fn parse_dont_enforce_required(serialized: &[u8]) -> ::std::result::Result<Self, ::protobuf::ParseError> {
    Self::parse_dont_enforce_required(serialized)
  }
}

impl ::std::fmt::Debug for PayloadLobbyStartGameRequest {
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

impl ::protobuf::Serialize for PayloadLobbyStartGameRequest {
  fn serialize(&self) -> ::std::result::Result<Vec<u8>, ::protobuf::SerializeError> {
    ::protobuf::AsView::as_view(self).serialize()
  }
}

// SAFETY:
// - `PayloadLobbyStartGameRequest` is `Sync` because it does not implement interior mutability.
//    Neither does `PayloadLobbyStartGameRequestMut`.
unsafe impl Sync for PayloadLobbyStartGameRequest {}

// SAFETY:
// - `PayloadLobbyStartGameRequest` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl Send for PayloadLobbyStartGameRequest {}

impl ::protobuf::Proxied for PayloadLobbyStartGameRequest {
  type View<'msg> = PayloadLobbyStartGameRequestView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for PayloadLobbyStartGameRequest {}

impl ::protobuf::MutProxied for PayloadLobbyStartGameRequest {
  type Mut<'msg> = PayloadLobbyStartGameRequestMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct PayloadLobbyStartGameRequestView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, PayloadLobbyStartGameRequest>,
  _phantom: ::std::marker::PhantomData<&'msg ()>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for PayloadLobbyStartGameRequestView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for PayloadLobbyStartGameRequestView<'msg> {
  type Message = PayloadLobbyStartGameRequest;
}

impl ::std::fmt::Debug for PayloadLobbyStartGameRequestView<'_> {
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

impl ::protobuf::Serialize for PayloadLobbyStartGameRequestView<'_> {
  fn serialize(&self) -> ::std::result::Result<Vec<u8>, ::protobuf::SerializeError> {
    // SAFETY: `MINI_TABLE` is the one associated with `self.raw_msg()`.
    let encoded = unsafe {
      ::protobuf::__internal::runtime::wire::encode(self.raw_msg(),
          <Self as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table())
    };
    encoded.map_err(|_| ::protobuf::SerializeError)
  }
}

impl ::std::default::Default for PayloadLobbyStartGameRequestView<'_> {
  fn default() -> PayloadLobbyStartGameRequestView<'static> {
    let inner = unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap_raw(::protobuf::__internal::runtime::ScratchSpace::zeroed_block()) };
    PayloadLobbyStartGameRequestView::new(::protobuf::__internal::Private, inner)
  }
}

#[allow(dead_code)]
impl<'msg> PayloadLobbyStartGameRequestView<'msg> {
  #[doc(hidden)]
  pub fn new(_private: ::protobuf::__internal::Private, inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, PayloadLobbyStartGameRequest>) -> Self {
    Self { inner, _phantom: ::std::marker::PhantomData }
  }

  fn raw_msg(&self) -> ::protobuf::__internal::runtime::RawMessage {
    self.inner.raw()
  }

  pub fn to_owned(&self) -> PayloadLobbyStartGameRequest {
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
// - `PayloadLobbyStartGameRequestView` is `Sync` because it does not support mutation.
unsafe impl Sync for PayloadLobbyStartGameRequestView<'_> {}

// SAFETY:
// - `PayloadLobbyStartGameRequestView` is `Send` because while its alive a `PayloadLobbyStartGameRequestMut` cannot.
// - `PayloadLobbyStartGameRequestView` does not use thread-local data.
unsafe impl Send for PayloadLobbyStartGameRequestView<'_> {}

impl<'msg> ::protobuf::Proxy<'msg> for PayloadLobbyStartGameRequestView<'msg> {}
impl<'msg> ::protobuf::ViewProxy<'msg> for PayloadLobbyStartGameRequestView<'msg> {}

impl<'msg> ::protobuf::AsView for PayloadLobbyStartGameRequestView<'msg> {
  type Proxied = PayloadLobbyStartGameRequest;
  fn as_view(&self) -> ::protobuf::View<'msg, PayloadLobbyStartGameRequest> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for PayloadLobbyStartGameRequestView<'msg> {
  fn into_view<'shorter>(self) -> PayloadLobbyStartGameRequestView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<PayloadLobbyStartGameRequest> for PayloadLobbyStartGameRequestView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> PayloadLobbyStartGameRequest {
    let mut dst = PayloadLobbyStartGameRequest::new();
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

impl<'msg> ::protobuf::IntoProxied<PayloadLobbyStartGameRequest> for PayloadLobbyStartGameRequestMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> PayloadLobbyStartGameRequest {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::runtime::UpbTypeConversions for PayloadLobbyStartGameRequest {
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
        PayloadLobbyStartGameRequestView::new(::protobuf::__internal::Private, inner)
    }

    unsafe fn from_message_mut<'msg>(msg: ::protobuf::__internal::runtime::RawMessage, arena: &'msg ::protobuf::__internal::runtime::Arena)
        -> PayloadLobbyStartGameRequestMut<'msg> {
        let inner = unsafe { ::protobuf::__internal::runtime::MessageMutInner::<'msg, PayloadLobbyStartGameRequest>::wrap_raw(msg, arena) };
        PayloadLobbyStartGameRequestMut::new(::protobuf::__internal::Private, inner)
    }
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct PayloadLobbyStartGameRequestMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, PayloadLobbyStartGameRequest>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for PayloadLobbyStartGameRequestMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for PayloadLobbyStartGameRequestMut<'msg> {
  type Message = PayloadLobbyStartGameRequest;
}

impl ::std::fmt::Debug for PayloadLobbyStartGameRequestMut<'_> {
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

impl ::protobuf::Serialize for PayloadLobbyStartGameRequestMut<'_> {
  fn serialize(&self) -> ::std::result::Result<Vec<u8>, ::protobuf::SerializeError> {
    ::protobuf::AsView::as_view(self).serialize()
  }
}

#[allow(dead_code)]
impl<'msg> PayloadLobbyStartGameRequestMut<'msg> {
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
  pub fn new(_private: ::protobuf::__internal::Private, inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, PayloadLobbyStartGameRequest>) -> Self {
    Self { inner }
  }

  fn raw_msg(&self) -> ::protobuf::__internal::runtime::RawMessage {
    self.inner.raw()
  }

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, PayloadLobbyStartGameRequest> {
    self.inner
  }

  pub fn to_owned(&self) -> PayloadLobbyStartGameRequest {
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
// - `PayloadLobbyStartGameRequestMut` does not perform any shared mutation.
// - `PayloadLobbyStartGameRequestMut` is not `Send`, and so even in the presence of mutator
//   splitting, synchronous access of an arena is impossible.
unsafe impl Sync for PayloadLobbyStartGameRequestMut<'_> {}

impl<'msg> ::protobuf::Proxy<'msg> for PayloadLobbyStartGameRequestMut<'msg> {}
impl<'msg> ::protobuf::MutProxy<'msg> for PayloadLobbyStartGameRequestMut<'msg> {}

impl<'msg> ::protobuf::AsView for PayloadLobbyStartGameRequestMut<'msg> {
  type Proxied = PayloadLobbyStartGameRequest;
  fn as_view(&self) -> ::protobuf::View<'_, PayloadLobbyStartGameRequest> {
    PayloadLobbyStartGameRequestView {
      inner: ::protobuf::__internal::runtime::MessageViewInner::view_of_mut(self.inner.clone()),
      _phantom: ::std::marker::PhantomData
    }
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for PayloadLobbyStartGameRequestMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, PayloadLobbyStartGameRequest>
  where
      'msg: 'shorter {
    PayloadLobbyStartGameRequestView {
      inner: ::protobuf::__internal::runtime::MessageViewInner::view_of_mut(self.inner.clone()),
      _phantom: ::std::marker::PhantomData
    }
  }
}

impl<'msg> ::protobuf::AsMut for PayloadLobbyStartGameRequestMut<'msg> {
  type MutProxied = PayloadLobbyStartGameRequest;
  fn as_mut(&mut self) -> PayloadLobbyStartGameRequestMut<'msg> {
    PayloadLobbyStartGameRequestMut { inner: self.inner }
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for PayloadLobbyStartGameRequestMut<'msg> {
  fn into_mut<'shorter>(self) -> PayloadLobbyStartGameRequestMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl PayloadLobbyStartGameRequest {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }

  fn raw_msg(&self) -> ::protobuf::__internal::runtime::RawMessage {
    self.inner.raw()
  }

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, PayloadLobbyStartGameRequest> {
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

  pub fn as_view(&self) -> PayloadLobbyStartGameRequestView {
    PayloadLobbyStartGameRequestView::new(
        ::protobuf::__internal::Private,
        ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner))
  }

  pub fn as_mut(&mut self) -> PayloadLobbyStartGameRequestMut {
    let inner = ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner);
    PayloadLobbyStartGameRequestMut::new(::protobuf::__internal::Private, inner)
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

}  // impl PayloadLobbyStartGameRequest

impl ::std::ops::Drop for PayloadLobbyStartGameRequest {
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for PayloadLobbyStartGameRequest {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for PayloadLobbyStartGameRequest {
  type Proxied = Self;
  fn as_view(&self) -> PayloadLobbyStartGameRequestView {
    self.as_view()
  }
}

impl ::protobuf::AsMut for PayloadLobbyStartGameRequest {
  type MutProxied = Self;
  fn as_mut(&mut self) -> PayloadLobbyStartGameRequestMut {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for PayloadLobbyStartGameRequest {
  fn mini_table() -> *const ::protobuf::__internal::runtime::upb_MiniTable {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTablePtr> =
        ::std::sync::OnceLock::new();
    ONCE_LOCK.get_or_init(|| unsafe {
      super::Proto__PayloadLobbyStartGameRequest_msg_init.0 =
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
          super::Proto__PayloadLobbyStartGameRequest_msg_init.0,
          submessages.as_ptr() as *const *const ::protobuf::__internal::runtime::upb_MiniTable,
          submessages.len(), subenums.as_ptr(), subenums.len()));
      ::protobuf::__internal::runtime::MiniTablePtr(super::Proto__PayloadLobbyStartGameRequest_msg_init.0)
    }).0
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for PayloadLobbyStartGameRequest {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for PayloadLobbyStartGameRequestView<'_> {
  #[inline(always)]
  fn mini_table() -> *const ::protobuf::__internal::runtime::upb_MiniTable {
    <PayloadLobbyStartGameRequest as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for PayloadLobbyStartGameRequestMut<'_> {
  #[inline(always)]
  fn mini_table() -> *const ::protobuf::__internal::runtime::upb_MiniTable {
    <PayloadLobbyStartGameRequest as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for PayloadLobbyStartGameRequest {
  type Msg = PayloadLobbyStartGameRequest;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<PayloadLobbyStartGameRequest> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for PayloadLobbyStartGameRequest {
  type Msg = PayloadLobbyStartGameRequest;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<PayloadLobbyStartGameRequest> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for PayloadLobbyStartGameRequestMut<'_> {
  type Msg = PayloadLobbyStartGameRequest;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<PayloadLobbyStartGameRequest> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for PayloadLobbyStartGameRequestMut<'_> {
  type Msg = PayloadLobbyStartGameRequest;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<PayloadLobbyStartGameRequest> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for PayloadLobbyStartGameRequestView<'_> {
  type Msg = PayloadLobbyStartGameRequest;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<PayloadLobbyStartGameRequest> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for PayloadLobbyStartGameRequestMut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}


// upb kernel doesn't support any owned message or message mut interop.
impl ::protobuf::OwnedMessageInterop for PayloadLobbyStartGameRequest {}
impl<'a> ::protobuf::MessageMutInterop<'a> for PayloadLobbyStartGameRequestMut<'a> {}

impl<'a> ::protobuf::MessageViewInterop<'a> for PayloadLobbyStartGameRequestView<'a> {
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
pub(crate) static mut Proto__PayloadLobbyStartGameResponse_msg_init: ::protobuf::__internal::runtime::MiniTablePtr =
    ::protobuf::__internal::runtime::MiniTablePtr(::std::ptr::null_mut());
#[allow(non_camel_case_types)]
pub struct PayloadLobbyStartGameResponse {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<PayloadLobbyStartGameResponse>
}

impl ::protobuf::Message for PayloadLobbyStartGameResponse {}

impl ::std::default::Default for PayloadLobbyStartGameResponse {
  fn default() -> Self {
    Self::new()
  }
}

impl ::protobuf::Parse for PayloadLobbyStartGameResponse {
  fn parse(serialized: &[u8]) -> ::std::result::Result<Self, ::protobuf::ParseError> {
    Self::parse(serialized)
  }

  fn parse_dont_enforce_required(serialized: &[u8]) -> ::std::result::Result<Self, ::protobuf::ParseError> {
    Self::parse_dont_enforce_required(serialized)
  }
}

impl ::std::fmt::Debug for PayloadLobbyStartGameResponse {
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

impl ::protobuf::Serialize for PayloadLobbyStartGameResponse {
  fn serialize(&self) -> ::std::result::Result<Vec<u8>, ::protobuf::SerializeError> {
    ::protobuf::AsView::as_view(self).serialize()
  }
}

// SAFETY:
// - `PayloadLobbyStartGameResponse` is `Sync` because it does not implement interior mutability.
//    Neither does `PayloadLobbyStartGameResponseMut`.
unsafe impl Sync for PayloadLobbyStartGameResponse {}

// SAFETY:
// - `PayloadLobbyStartGameResponse` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl Send for PayloadLobbyStartGameResponse {}

impl ::protobuf::Proxied for PayloadLobbyStartGameResponse {
  type View<'msg> = PayloadLobbyStartGameResponseView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for PayloadLobbyStartGameResponse {}

impl ::protobuf::MutProxied for PayloadLobbyStartGameResponse {
  type Mut<'msg> = PayloadLobbyStartGameResponseMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct PayloadLobbyStartGameResponseView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, PayloadLobbyStartGameResponse>,
  _phantom: ::std::marker::PhantomData<&'msg ()>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for PayloadLobbyStartGameResponseView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for PayloadLobbyStartGameResponseView<'msg> {
  type Message = PayloadLobbyStartGameResponse;
}

impl ::std::fmt::Debug for PayloadLobbyStartGameResponseView<'_> {
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

impl ::protobuf::Serialize for PayloadLobbyStartGameResponseView<'_> {
  fn serialize(&self) -> ::std::result::Result<Vec<u8>, ::protobuf::SerializeError> {
    // SAFETY: `MINI_TABLE` is the one associated with `self.raw_msg()`.
    let encoded = unsafe {
      ::protobuf::__internal::runtime::wire::encode(self.raw_msg(),
          <Self as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table())
    };
    encoded.map_err(|_| ::protobuf::SerializeError)
  }
}

impl ::std::default::Default for PayloadLobbyStartGameResponseView<'_> {
  fn default() -> PayloadLobbyStartGameResponseView<'static> {
    let inner = unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap_raw(::protobuf::__internal::runtime::ScratchSpace::zeroed_block()) };
    PayloadLobbyStartGameResponseView::new(::protobuf::__internal::Private, inner)
  }
}

#[allow(dead_code)]
impl<'msg> PayloadLobbyStartGameResponseView<'msg> {
  #[doc(hidden)]
  pub fn new(_private: ::protobuf::__internal::Private, inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, PayloadLobbyStartGameResponse>) -> Self {
    Self { inner, _phantom: ::std::marker::PhantomData }
  }

  fn raw_msg(&self) -> ::protobuf::__internal::runtime::RawMessage {
    self.inner.raw()
  }

  pub fn to_owned(&self) -> PayloadLobbyStartGameResponse {
    ::protobuf::IntoProxied::into_proxied(*self, ::protobuf::__internal::Private)
  }

  // zone_id: optional uint64
  pub fn zone_id(self) -> u64 {
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

  // player_entity_id: optional uint64
  pub fn player_entity_id(self) -> u64 {
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

  // position: optional message Proto.Vector3
  pub fn has_position(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(2)
    }
  }
  pub fn position_opt(self) -> ::protobuf::Optional<super::Vector3View<'msg>> {
        ::protobuf::Optional::new(self.position(), self.has_position())
  }
  pub fn position(self) -> super::Vector3View<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(2)
    };
    let raw = submsg.map(|ptr| ptr.raw()).unwrap_or(::protobuf::__internal::runtime::ScratchSpace::zeroed_block());
    let inner = unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap_raw(raw) };
    super::Vector3View::new(::protobuf::__internal::Private, inner)
  }

}

// SAFETY:
// - `PayloadLobbyStartGameResponseView` is `Sync` because it does not support mutation.
unsafe impl Sync for PayloadLobbyStartGameResponseView<'_> {}

// SAFETY:
// - `PayloadLobbyStartGameResponseView` is `Send` because while its alive a `PayloadLobbyStartGameResponseMut` cannot.
// - `PayloadLobbyStartGameResponseView` does not use thread-local data.
unsafe impl Send for PayloadLobbyStartGameResponseView<'_> {}

impl<'msg> ::protobuf::Proxy<'msg> for PayloadLobbyStartGameResponseView<'msg> {}
impl<'msg> ::protobuf::ViewProxy<'msg> for PayloadLobbyStartGameResponseView<'msg> {}

impl<'msg> ::protobuf::AsView for PayloadLobbyStartGameResponseView<'msg> {
  type Proxied = PayloadLobbyStartGameResponse;
  fn as_view(&self) -> ::protobuf::View<'msg, PayloadLobbyStartGameResponse> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for PayloadLobbyStartGameResponseView<'msg> {
  fn into_view<'shorter>(self) -> PayloadLobbyStartGameResponseView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<PayloadLobbyStartGameResponse> for PayloadLobbyStartGameResponseView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> PayloadLobbyStartGameResponse {
    let mut dst = PayloadLobbyStartGameResponse::new();
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

impl<'msg> ::protobuf::IntoProxied<PayloadLobbyStartGameResponse> for PayloadLobbyStartGameResponseMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> PayloadLobbyStartGameResponse {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::runtime::UpbTypeConversions for PayloadLobbyStartGameResponse {
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
        PayloadLobbyStartGameResponseView::new(::protobuf::__internal::Private, inner)
    }

    unsafe fn from_message_mut<'msg>(msg: ::protobuf::__internal::runtime::RawMessage, arena: &'msg ::protobuf::__internal::runtime::Arena)
        -> PayloadLobbyStartGameResponseMut<'msg> {
        let inner = unsafe { ::protobuf::__internal::runtime::MessageMutInner::<'msg, PayloadLobbyStartGameResponse>::wrap_raw(msg, arena) };
        PayloadLobbyStartGameResponseMut::new(::protobuf::__internal::Private, inner)
    }
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct PayloadLobbyStartGameResponseMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, PayloadLobbyStartGameResponse>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for PayloadLobbyStartGameResponseMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for PayloadLobbyStartGameResponseMut<'msg> {
  type Message = PayloadLobbyStartGameResponse;
}

impl ::std::fmt::Debug for PayloadLobbyStartGameResponseMut<'_> {
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

impl ::protobuf::Serialize for PayloadLobbyStartGameResponseMut<'_> {
  fn serialize(&self) -> ::std::result::Result<Vec<u8>, ::protobuf::SerializeError> {
    ::protobuf::AsView::as_view(self).serialize()
  }
}

#[allow(dead_code)]
impl<'msg> PayloadLobbyStartGameResponseMut<'msg> {
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
  pub fn new(_private: ::protobuf::__internal::Private, inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, PayloadLobbyStartGameResponse>) -> Self {
    Self { inner }
  }

  fn raw_msg(&self) -> ::protobuf::__internal::runtime::RawMessage {
    self.inner.raw()
  }

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, PayloadLobbyStartGameResponse> {
    self.inner
  }

  pub fn to_owned(&self) -> PayloadLobbyStartGameResponse {
    ::protobuf::AsView::as_view(self).to_owned()
  }

  fn arena(&mut self) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }

  // zone_id: optional uint64
  pub fn zone_id(&self) -> u64 {
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
  pub fn set_zone_id(&mut self, val: u64) {
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

  // player_entity_id: optional uint64
  pub fn player_entity_id(&self) -> u64 {
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
  pub fn set_player_entity_id(&mut self, val: u64) {
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

  // position: optional message Proto.Vector3
  pub fn has_position(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(2)
    }
  }
  pub fn clear_position(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        2
      );
    }
  }
  pub fn position_opt(&self) -> ::protobuf::Optional<super::Vector3View<'_>> {
        ::protobuf::Optional::new(self.position(), self.has_position())
  }
  pub fn position(&self) -> super::Vector3View<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(2)
    };
    let raw = submsg.map(|ptr| ptr.raw()).unwrap_or(::protobuf::__internal::runtime::ScratchSpace::zeroed_block());
    let inner = unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap_raw(raw) };
    super::Vector3View::new(::protobuf::__internal::Private, inner)
  }
  pub fn position_mut(&mut self) -> super::Vector3Mut<'_> {
     let ptr = unsafe {
       self.inner.ptr_mut().get_or_create_mutable_message_at_index(
         2, self.arena()
       ).unwrap()
     };
     super::Vector3Mut::from_parent(
       ::protobuf::__internal::Private,
       self.as_message_mut_inner(::protobuf::__internal::Private),
       ptr.raw())
  }
  pub fn set_position(&mut self,
    val: impl ::protobuf::IntoProxied<super::Vector3>) {

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
        2, child_ptr
      );
    }
  }

}

// SAFETY:
// - `PayloadLobbyStartGameResponseMut` does not perform any shared mutation.
// - `PayloadLobbyStartGameResponseMut` is not `Send`, and so even in the presence of mutator
//   splitting, synchronous access of an arena is impossible.
unsafe impl Sync for PayloadLobbyStartGameResponseMut<'_> {}

impl<'msg> ::protobuf::Proxy<'msg> for PayloadLobbyStartGameResponseMut<'msg> {}
impl<'msg> ::protobuf::MutProxy<'msg> for PayloadLobbyStartGameResponseMut<'msg> {}

impl<'msg> ::protobuf::AsView for PayloadLobbyStartGameResponseMut<'msg> {
  type Proxied = PayloadLobbyStartGameResponse;
  fn as_view(&self) -> ::protobuf::View<'_, PayloadLobbyStartGameResponse> {
    PayloadLobbyStartGameResponseView {
      inner: ::protobuf::__internal::runtime::MessageViewInner::view_of_mut(self.inner.clone()),
      _phantom: ::std::marker::PhantomData
    }
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for PayloadLobbyStartGameResponseMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, PayloadLobbyStartGameResponse>
  where
      'msg: 'shorter {
    PayloadLobbyStartGameResponseView {
      inner: ::protobuf::__internal::runtime::MessageViewInner::view_of_mut(self.inner.clone()),
      _phantom: ::std::marker::PhantomData
    }
  }
}

impl<'msg> ::protobuf::AsMut for PayloadLobbyStartGameResponseMut<'msg> {
  type MutProxied = PayloadLobbyStartGameResponse;
  fn as_mut(&mut self) -> PayloadLobbyStartGameResponseMut<'msg> {
    PayloadLobbyStartGameResponseMut { inner: self.inner }
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for PayloadLobbyStartGameResponseMut<'msg> {
  fn into_mut<'shorter>(self) -> PayloadLobbyStartGameResponseMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl PayloadLobbyStartGameResponse {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }

  fn raw_msg(&self) -> ::protobuf::__internal::runtime::RawMessage {
    self.inner.raw()
  }

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, PayloadLobbyStartGameResponse> {
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

  pub fn as_view(&self) -> PayloadLobbyStartGameResponseView {
    PayloadLobbyStartGameResponseView::new(
        ::protobuf::__internal::Private,
        ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner))
  }

  pub fn as_mut(&mut self) -> PayloadLobbyStartGameResponseMut {
    let inner = ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner);
    PayloadLobbyStartGameResponseMut::new(::protobuf::__internal::Private, inner)
  }

  // zone_id: optional uint64
  pub fn zone_id(&self) -> u64 {
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
  pub fn set_zone_id(&mut self, val: u64) {
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

  // player_entity_id: optional uint64
  pub fn player_entity_id(&self) -> u64 {
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
  pub fn set_player_entity_id(&mut self, val: u64) {
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

  // position: optional message Proto.Vector3
  pub fn has_position(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(2)
    }
  }
  pub fn clear_position(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        2
      );
    }
  }
  pub fn position_opt(&self) -> ::protobuf::Optional<super::Vector3View<'_>> {
        ::protobuf::Optional::new(self.position(), self.has_position())
  }
  pub fn position(&self) -> super::Vector3View<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(2)
    };
    let raw = submsg.map(|ptr| ptr.raw()).unwrap_or(::protobuf::__internal::runtime::ScratchSpace::zeroed_block());
    let inner = unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap_raw(raw) };
    super::Vector3View::new(::protobuf::__internal::Private, inner)
  }
  pub fn position_mut(&mut self) -> super::Vector3Mut<'_> {
     let ptr = unsafe {
       self.inner.ptr_mut().get_or_create_mutable_message_at_index(
         2, self.arena()
       ).unwrap()
     };
     super::Vector3Mut::from_parent(
       ::protobuf::__internal::Private,
       self.as_message_mut_inner(::protobuf::__internal::Private),
       ptr.raw())
  }
  pub fn set_position(&mut self,
    val: impl ::protobuf::IntoProxied<super::Vector3>) {

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
        2, child_ptr
      );
    }
  }

}  // impl PayloadLobbyStartGameResponse

impl ::std::ops::Drop for PayloadLobbyStartGameResponse {
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for PayloadLobbyStartGameResponse {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for PayloadLobbyStartGameResponse {
  type Proxied = Self;
  fn as_view(&self) -> PayloadLobbyStartGameResponseView {
    self.as_view()
  }
}

impl ::protobuf::AsMut for PayloadLobbyStartGameResponse {
  type MutProxied = Self;
  fn as_mut(&mut self) -> PayloadLobbyStartGameResponseMut {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for PayloadLobbyStartGameResponse {
  fn mini_table() -> *const ::protobuf::__internal::runtime::upb_MiniTable {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTablePtr> =
        ::std::sync::OnceLock::new();
    ONCE_LOCK.get_or_init(|| unsafe {
      super::Proto__PayloadLobbyStartGameResponse_msg_init.0 =
          ::protobuf::__internal::runtime::upb_MiniTable_Build(
              "$,P,P3".as_ptr(),
              6,
              ::protobuf::__internal::runtime::THREAD_LOCAL_ARENA.with(|a| a.raw()),
              ::std::ptr::null_mut());
      let submessages = [
        <super::Vector3 as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
      ];
      let subenums = [
      ];
      assert!(::protobuf::__internal::runtime::upb_MiniTable_Link(
          super::Proto__PayloadLobbyStartGameResponse_msg_init.0,
          submessages.as_ptr() as *const *const ::protobuf::__internal::runtime::upb_MiniTable,
          submessages.len(), subenums.as_ptr(), subenums.len()));
      ::protobuf::__internal::runtime::MiniTablePtr(super::Proto__PayloadLobbyStartGameResponse_msg_init.0)
    }).0
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for PayloadLobbyStartGameResponse {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for PayloadLobbyStartGameResponseView<'_> {
  #[inline(always)]
  fn mini_table() -> *const ::protobuf::__internal::runtime::upb_MiniTable {
    <PayloadLobbyStartGameResponse as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for PayloadLobbyStartGameResponseMut<'_> {
  #[inline(always)]
  fn mini_table() -> *const ::protobuf::__internal::runtime::upb_MiniTable {
    <PayloadLobbyStartGameResponse as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for PayloadLobbyStartGameResponse {
  type Msg = PayloadLobbyStartGameResponse;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<PayloadLobbyStartGameResponse> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for PayloadLobbyStartGameResponse {
  type Msg = PayloadLobbyStartGameResponse;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<PayloadLobbyStartGameResponse> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for PayloadLobbyStartGameResponseMut<'_> {
  type Msg = PayloadLobbyStartGameResponse;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<PayloadLobbyStartGameResponse> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for PayloadLobbyStartGameResponseMut<'_> {
  type Msg = PayloadLobbyStartGameResponse;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<PayloadLobbyStartGameResponse> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for PayloadLobbyStartGameResponseView<'_> {
  type Msg = PayloadLobbyStartGameResponse;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<PayloadLobbyStartGameResponse> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for PayloadLobbyStartGameResponseMut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}


// upb kernel doesn't support any owned message or message mut interop.
impl ::protobuf::OwnedMessageInterop for PayloadLobbyStartGameResponse {}
impl<'a> ::protobuf::MessageMutInterop<'a> for PayloadLobbyStartGameResponseMut<'a> {}

impl<'a> ::protobuf::MessageViewInterop<'a> for PayloadLobbyStartGameResponseView<'a> {
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

