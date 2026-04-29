const _: () = ::protobuf::__internal::assert_compatible_gencode_version("4.32.1-release");
// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut Proto__RoutePlayerData_msg_init: ::protobuf::__internal::runtime::MiniTablePtr =
    ::protobuf::__internal::runtime::MiniTablePtr(::std::ptr::null_mut());
#[allow(non_camel_case_types)]
pub struct RoutePlayerData {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<RoutePlayerData>
}

impl ::protobuf::Message for RoutePlayerData {}

impl ::std::default::Default for RoutePlayerData {
  fn default() -> Self {
    Self::new()
  }
}

impl ::protobuf::Parse for RoutePlayerData {
  fn parse(serialized: &[u8]) -> ::std::result::Result<Self, ::protobuf::ParseError> {
    Self::parse(serialized)
  }

  fn parse_dont_enforce_required(serialized: &[u8]) -> ::std::result::Result<Self, ::protobuf::ParseError> {
    Self::parse_dont_enforce_required(serialized)
  }
}

impl ::std::fmt::Debug for RoutePlayerData {
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

impl ::protobuf::Serialize for RoutePlayerData {
  fn serialize(&self) -> ::std::result::Result<Vec<u8>, ::protobuf::SerializeError> {
    ::protobuf::AsView::as_view(self).serialize()
  }
}

// SAFETY:
// - `RoutePlayerData` is `Sync` because it does not implement interior mutability.
//    Neither does `RoutePlayerDataMut`.
unsafe impl Sync for RoutePlayerData {}

// SAFETY:
// - `RoutePlayerData` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl Send for RoutePlayerData {}

impl ::protobuf::Proxied for RoutePlayerData {
  type View<'msg> = RoutePlayerDataView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for RoutePlayerData {}

impl ::protobuf::MutProxied for RoutePlayerData {
  type Mut<'msg> = RoutePlayerDataMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct RoutePlayerDataView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, RoutePlayerData>,
  _phantom: ::std::marker::PhantomData<&'msg ()>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for RoutePlayerDataView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for RoutePlayerDataView<'msg> {
  type Message = RoutePlayerData;
}

impl ::std::fmt::Debug for RoutePlayerDataView<'_> {
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

impl ::protobuf::Serialize for RoutePlayerDataView<'_> {
  fn serialize(&self) -> ::std::result::Result<Vec<u8>, ::protobuf::SerializeError> {
    // SAFETY: `MINI_TABLE` is the one associated with `self.raw_msg()`.
    let encoded = unsafe {
      ::protobuf::__internal::runtime::wire::encode(self.raw_msg(),
          <Self as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table())
    };
    encoded.map_err(|_| ::protobuf::SerializeError)
  }
}

impl ::std::default::Default for RoutePlayerDataView<'_> {
  fn default() -> RoutePlayerDataView<'static> {
    let inner = unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap_raw(::protobuf::__internal::runtime::ScratchSpace::zeroed_block()) };
    RoutePlayerDataView::new(::protobuf::__internal::Private, inner)
  }
}

#[allow(dead_code)]
impl<'msg> RoutePlayerDataView<'msg> {
  #[doc(hidden)]
  pub fn new(_private: ::protobuf::__internal::Private, inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, RoutePlayerData>) -> Self {
    Self { inner, _phantom: ::std::marker::PhantomData }
  }

  fn raw_msg(&self) -> ::protobuf::__internal::runtime::RawMessage {
    self.inner.raw()
  }

  pub fn to_owned(&self) -> RoutePlayerData {
    ::protobuf::IntoProxied::into_proxied(*self, ::protobuf::__internal::Private)
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
        0, (0u64).into()
      ).try_into().unwrap()
    }
  }

  // position: optional message Proto.Vector3
  pub fn has_position(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn position_opt(self) -> ::protobuf::Optional<super::Vector3View<'msg>> {
        ::protobuf::Optional::new(self.position(), self.has_position())
  }
  pub fn position(self) -> super::Vector3View<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(1)
    };
    let raw = submsg.map(|ptr| ptr.raw()).unwrap_or(::protobuf::__internal::runtime::ScratchSpace::zeroed_block());
    let inner = unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap_raw(raw) };
    super::Vector3View::new(::protobuf::__internal::Private, inner)
  }

}

// SAFETY:
// - `RoutePlayerDataView` is `Sync` because it does not support mutation.
unsafe impl Sync for RoutePlayerDataView<'_> {}

// SAFETY:
// - `RoutePlayerDataView` is `Send` because while its alive a `RoutePlayerDataMut` cannot.
// - `RoutePlayerDataView` does not use thread-local data.
unsafe impl Send for RoutePlayerDataView<'_> {}

impl<'msg> ::protobuf::Proxy<'msg> for RoutePlayerDataView<'msg> {}
impl<'msg> ::protobuf::ViewProxy<'msg> for RoutePlayerDataView<'msg> {}

impl<'msg> ::protobuf::AsView for RoutePlayerDataView<'msg> {
  type Proxied = RoutePlayerData;
  fn as_view(&self) -> ::protobuf::View<'msg, RoutePlayerData> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for RoutePlayerDataView<'msg> {
  fn into_view<'shorter>(self) -> RoutePlayerDataView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<RoutePlayerData> for RoutePlayerDataView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> RoutePlayerData {
    let mut dst = RoutePlayerData::new();
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

impl<'msg> ::protobuf::IntoProxied<RoutePlayerData> for RoutePlayerDataMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> RoutePlayerData {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::runtime::UpbTypeConversions for RoutePlayerData {
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
        RoutePlayerDataView::new(::protobuf::__internal::Private, inner)
    }

    unsafe fn from_message_mut<'msg>(msg: ::protobuf::__internal::runtime::RawMessage, arena: &'msg ::protobuf::__internal::runtime::Arena)
        -> RoutePlayerDataMut<'msg> {
        let inner = unsafe { ::protobuf::__internal::runtime::MessageMutInner::<'msg, RoutePlayerData>::wrap_raw(msg, arena) };
        RoutePlayerDataMut::new(::protobuf::__internal::Private, inner)
    }
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct RoutePlayerDataMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, RoutePlayerData>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for RoutePlayerDataMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for RoutePlayerDataMut<'msg> {
  type Message = RoutePlayerData;
}

impl ::std::fmt::Debug for RoutePlayerDataMut<'_> {
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

impl ::protobuf::Serialize for RoutePlayerDataMut<'_> {
  fn serialize(&self) -> ::std::result::Result<Vec<u8>, ::protobuf::SerializeError> {
    ::protobuf::AsView::as_view(self).serialize()
  }
}

#[allow(dead_code)]
impl<'msg> RoutePlayerDataMut<'msg> {
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
  pub fn new(_private: ::protobuf::__internal::Private, inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, RoutePlayerData>) -> Self {
    Self { inner }
  }

  fn raw_msg(&self) -> ::protobuf::__internal::runtime::RawMessage {
    self.inner.raw()
  }

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, RoutePlayerData> {
    self.inner
  }

  pub fn to_owned(&self) -> RoutePlayerData {
    ::protobuf::AsView::as_view(self).to_owned()
  }

  fn arena(&mut self) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
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
        0, (0u64).into()
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
        0, val.into()
      )
    }
  }

  // position: optional message Proto.Vector3
  pub fn has_position(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn clear_position(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        1
      );
    }
  }
  pub fn position_opt(&self) -> ::protobuf::Optional<super::Vector3View<'_>> {
        ::protobuf::Optional::new(self.position(), self.has_position())
  }
  pub fn position(&self) -> super::Vector3View<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(1)
    };
    let raw = submsg.map(|ptr| ptr.raw()).unwrap_or(::protobuf::__internal::runtime::ScratchSpace::zeroed_block());
    let inner = unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap_raw(raw) };
    super::Vector3View::new(::protobuf::__internal::Private, inner)
  }
  pub fn position_mut(&mut self) -> super::Vector3Mut<'_> {
     let ptr = unsafe {
       self.inner.ptr_mut().get_or_create_mutable_message_at_index(
         1, self.arena()
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
        1, child_ptr
      );
    }
  }

}

// SAFETY:
// - `RoutePlayerDataMut` does not perform any shared mutation.
// - `RoutePlayerDataMut` is not `Send`, and so even in the presence of mutator
//   splitting, synchronous access of an arena is impossible.
unsafe impl Sync for RoutePlayerDataMut<'_> {}

impl<'msg> ::protobuf::Proxy<'msg> for RoutePlayerDataMut<'msg> {}
impl<'msg> ::protobuf::MutProxy<'msg> for RoutePlayerDataMut<'msg> {}

impl<'msg> ::protobuf::AsView for RoutePlayerDataMut<'msg> {
  type Proxied = RoutePlayerData;
  fn as_view(&self) -> ::protobuf::View<'_, RoutePlayerData> {
    RoutePlayerDataView {
      inner: ::protobuf::__internal::runtime::MessageViewInner::view_of_mut(self.inner.clone()),
      _phantom: ::std::marker::PhantomData
    }
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for RoutePlayerDataMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, RoutePlayerData>
  where
      'msg: 'shorter {
    RoutePlayerDataView {
      inner: ::protobuf::__internal::runtime::MessageViewInner::view_of_mut(self.inner.clone()),
      _phantom: ::std::marker::PhantomData
    }
  }
}

impl<'msg> ::protobuf::AsMut for RoutePlayerDataMut<'msg> {
  type MutProxied = RoutePlayerData;
  fn as_mut(&mut self) -> RoutePlayerDataMut<'msg> {
    RoutePlayerDataMut { inner: self.inner }
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for RoutePlayerDataMut<'msg> {
  fn into_mut<'shorter>(self) -> RoutePlayerDataMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl RoutePlayerData {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }

  fn raw_msg(&self) -> ::protobuf::__internal::runtime::RawMessage {
    self.inner.raw()
  }

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, RoutePlayerData> {
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

  pub fn as_view(&self) -> RoutePlayerDataView {
    RoutePlayerDataView::new(
        ::protobuf::__internal::Private,
        ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner))
  }

  pub fn as_mut(&mut self) -> RoutePlayerDataMut {
    let inner = ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner);
    RoutePlayerDataMut::new(::protobuf::__internal::Private, inner)
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
        0, (0u64).into()
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
        0, val.into()
      )
    }
  }

  // position: optional message Proto.Vector3
  pub fn has_position(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn clear_position(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        1
      );
    }
  }
  pub fn position_opt(&self) -> ::protobuf::Optional<super::Vector3View<'_>> {
        ::protobuf::Optional::new(self.position(), self.has_position())
  }
  pub fn position(&self) -> super::Vector3View<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(1)
    };
    let raw = submsg.map(|ptr| ptr.raw()).unwrap_or(::protobuf::__internal::runtime::ScratchSpace::zeroed_block());
    let inner = unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap_raw(raw) };
    super::Vector3View::new(::protobuf::__internal::Private, inner)
  }
  pub fn position_mut(&mut self) -> super::Vector3Mut<'_> {
     let ptr = unsafe {
       self.inner.ptr_mut().get_or_create_mutable_message_at_index(
         1, self.arena()
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
        1, child_ptr
      );
    }
  }

}  // impl RoutePlayerData

impl ::std::ops::Drop for RoutePlayerData {
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for RoutePlayerData {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for RoutePlayerData {
  type Proxied = Self;
  fn as_view(&self) -> RoutePlayerDataView {
    self.as_view()
  }
}

impl ::protobuf::AsMut for RoutePlayerData {
  type MutProxied = Self;
  fn as_mut(&mut self) -> RoutePlayerDataMut {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for RoutePlayerData {
  fn mini_table() -> *const ::protobuf::__internal::runtime::upb_MiniTable {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTablePtr> =
        ::std::sync::OnceLock::new();
    ONCE_LOCK.get_or_init(|| unsafe {
      super::Proto__RoutePlayerData_msg_init.0 =
          ::protobuf::__internal::runtime::upb_MiniTable_Build(
              "$,P3".as_ptr(),
              4,
              ::protobuf::__internal::runtime::THREAD_LOCAL_ARENA.with(|a| a.raw()),
              ::std::ptr::null_mut());
      let submessages = [
        <super::Vector3 as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
      ];
      let subenums = [
      ];
      assert!(::protobuf::__internal::runtime::upb_MiniTable_Link(
          super::Proto__RoutePlayerData_msg_init.0,
          submessages.as_ptr() as *const *const ::protobuf::__internal::runtime::upb_MiniTable,
          submessages.len(), subenums.as_ptr(), subenums.len()));
      ::protobuf::__internal::runtime::MiniTablePtr(super::Proto__RoutePlayerData_msg_init.0)
    }).0
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for RoutePlayerData {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for RoutePlayerDataView<'_> {
  #[inline(always)]
  fn mini_table() -> *const ::protobuf::__internal::runtime::upb_MiniTable {
    <RoutePlayerData as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for RoutePlayerDataMut<'_> {
  #[inline(always)]
  fn mini_table() -> *const ::protobuf::__internal::runtime::upb_MiniTable {
    <RoutePlayerData as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for RoutePlayerData {
  type Msg = RoutePlayerData;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<RoutePlayerData> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for RoutePlayerData {
  type Msg = RoutePlayerData;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<RoutePlayerData> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for RoutePlayerDataMut<'_> {
  type Msg = RoutePlayerData;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<RoutePlayerData> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for RoutePlayerDataMut<'_> {
  type Msg = RoutePlayerData;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<RoutePlayerData> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for RoutePlayerDataView<'_> {
  type Msg = RoutePlayerData;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<RoutePlayerData> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for RoutePlayerDataMut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}


// upb kernel doesn't support any owned message or message mut interop.
impl ::protobuf::OwnedMessageInterop for RoutePlayerData {}
impl<'a> ::protobuf::MessageMutInterop<'a> for RoutePlayerDataMut<'a> {}

impl<'a> ::protobuf::MessageViewInterop<'a> for RoutePlayerDataView<'a> {
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

