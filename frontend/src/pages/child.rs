use crate::service::ChildService;
use chrono::NaiveDate;
use ogts_common::model::Child;
use ogts_common::ListOptions;
use patternfly_yew::prelude::*;
use yew::prelude::*;
use yew_hooks::{use_async_with_options, UseAsyncOptions};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ChildEntry(pub Child);

impl TableEntryRenderer for ChildEntry {
    fn render_cell(&self, context: &CellContext) -> Cell {
        match context.column {
            0 => html!(&self.0.family_name),
            1 => html!(&self.0.name),
            _ => html!(),
        }
        .into()
    }
}

#[function_component(ChildList)]
pub fn child_list() -> Html {
    let header = html_nested!(
      <TableHeader>
        <TableColumn label="Family Name"/>
        <TableColumn label="Name"/>
      </TableHeader>
    );

    let model = use_async_with_options(
        async {
            ChildService::new()
                .list(ListOptions::default())
                .await
                .map(|result| SharedTableModel::new(result.into_iter().map(ChildEntry).collect()))
                .map_err(|err| err.to_string())
            //Ok::<_, String>(SharedTableModel::new(entries))
        },
        UseAsyncOptions::enable_auto(),
    );

    let entries = match &model.data {
        Some(data) => data.clone(),
        None => SharedTableModel::default(),
    };

    let backdropper = use_backdrop();
    let onclick_add = {
        let backdropper = backdropper.clone();
        Callback::from(move |_| {
            if let Some(backdropper) = &backdropper {
                backdropper.open(html!(<ChildAdd/>));
            }
        })
    };

    html!(
        <PageSection>
            <Toolbar>
                <ToolbarItem>
                    <Button label="Add" variant={ButtonVariant::Primary} onclick={onclick_add} />
                </ToolbarItem>
                <ToolbarItem r#type={ToolbarItemType::Pagination} >
                    <Pagination/>
                </ToolbarItem>
            </Toolbar>
            <Table<SharedTableModel<ChildEntry>> {entries} {header} />
        </PageSection>
    )
}

#[function_component(ChildAdd)]
pub fn child_add() -> Html {
    use patternfly_yew::next::TextInput;

    let form_state = use_state_eq(InputState::default);

    let onvalidated_form = {
        let form_state = form_state.clone();
        Callback::from(move |state| form_state.set(state))
    };

    let validator_family_name = |ctx: ValidationContext<String>| {
        if ctx.value.is_empty() {
            return ValidationResult::error("Must not be empty");
        }

        ValidationResult::ok()
    };
    let family_name = use_state_eq(String::default);

    let onclick_create = Callback::from(|_| {});

    let footer = {
        html!(
            <Button
                variant={ButtonVariant::Primary}
                disabled={(*form_state) == InputState::Error}
                r#type={ButtonType::Submit}
                onclick={onclick_create}
                form="create-form"
            >
                {"Create"}
            </Button>
        )
    };

    html!(
        <Bullseye plain=true>
            <Modal
                title="Add Child"
                variant={ModalVariant::Medium}
                {footer}
            >
                <Form id="create-form" method="dialog"
                    onvalidated={onvalidated_form}
                >
                    <FormGroupValidated<TextInput>
                        label="Family Name"
                        required=true
                        validator={Validator::from(validator_family_name)}
                    >
                        <TextInput value={(*family_name).clone()} oninput={Callback::from(move |data| family_name.set(data))}/>
                    </FormGroupValidated<TextInput>>
                </Form>
            </Modal>
        </Bullseye>
    )
}
